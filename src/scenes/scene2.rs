// src/scenes/scene2.rs
use macroquad::{
    color::{BLACK, WHITE},
    input::KeyCode,
    prelude::*,
    rand::{gen_range, rand, srand},
};

use crate::{
    assets::*,
    dialogue::*,
    gui::text_box::{draw_text_background, TextBoxOpts},
    race::{corridor::{CorridorScroller, ScrollDir}, obstacle::Obstacle, player::Player}, types::GamePhase,
};

pub async fn scene2() -> GamePhase {
    let mut game_phase: GamePhase = GamePhase::Scene2Dialogue;
    let texture_sun = load_texture(SCENE_2_SUN).await.unwrap();
    fade_in_texture(&texture_sun, 3.5).await;

    loop {
        draw_text_background(
            "Pronto, el Sol abrasador quemará el ojo de Begitxo. A menos que... ¿DÓNDE ESTÁN SUS GAFAS? Corre hacia el escenario y esquiva los obstáculos. Salta con W o Espacio. (Enter)",
            TextBoxOpts { font_size: 50, color: WHITE, max_width: screen_width()*0.8, ..Default::default() },
        ).await;
        if are_keys_pressed(&[KeyCode::Escape, KeyCode::Enter]).await  {
            break; 
        }
    }

    let race_tex  = load_texture(SCENE_2_RACE).await.unwrap();
    let mut scroller = CorridorScroller::from_texture(race_tex, 350.0, ScrollDir::Right, 2);
    let player_tex   = load_texture(SCENE_2_PLAYER).await.unwrap();
    let mut player   = Player::new(player_tex);

    let obstacle_defs = &[
        (load_texture(SCENE_2_OBSTACLE_1).await.unwrap(), Vec2::new(45.0, 61.0)),
        (load_texture(SCENE_2_OBSTACLE_2).await.unwrap(), Vec2::new(90.0, 122.0)),
        // TO DO: Add more obstacles
    ];

    srand(get_time() as u64);
    let mut obstacles: Vec<Obstacle> = Vec::new();
    let mut spawn_timer = 1.1 * ((rand() % 5 + 1) as f32);

    loop {
        let dt = get_frame_time();

        if game_phase == GamePhase::Scene2Race { scroller.update(); } // move background
        if scroller.finished() { break; } // end of race

        if game_phase == GamePhase::Scene2Race {
            if are_keys_pressed(&[KeyCode::Space, KeyCode::W]).await  { 
                player.jump();
            }
        } else if are_keys_pressed(&[KeyCode::Escape, KeyCode::Enter]).await  {
            player.reset();
            obstacles.clear();
            spawn_timer = 1.0;
            game_phase = GamePhase::Scene2Race
        }

        if game_phase == GamePhase::Scene2Race {
            player.update(dt);

            // spawn
            spawn_timer -= dt;
            if spawn_timer <= 0.0 {
                spawn_timer = gen_range(0.9, 1.8);
                let (tex, size) = &obstacle_defs[rand() as usize % obstacle_defs.len()];
                let spawn_x = match scroller.dir {
                    ScrollDir::Left  => screen_width() + 32.0,
                    ScrollDir::Right => -size.x - 32.0,
                };
                obstacles.push(Obstacle::new(spawn_x, tex.clone(), *size));
            }

            // move & cull
            for obs in &mut obstacles { obs.update(scroller.speed, scroller.dir, dt); }
            obstacles.retain(|o| !o.offscreen(scroller.dir));

            // collision
            if obstacles.iter().any(|o| o.rect().overlaps(&player.hitbox())) { 
                game_phase = GamePhase::Scene2Collision;
            }
        }

        clear_background(BLACK);
        scroller.draw();
        player.draw();
        for o in &obstacles { 
            o.draw(); 
            // DEBUG: draw the obstacle's hitbox
            //let r = o.rect();
            //draw_rectangle_lines(
            //    r.x, r.y, r.w, r.h,
            //    2.0,   // line thickness
            //    RED,  // color
            //);
        }
        // DEBUG: draw the player's hitbox
        //let pr = player.hitbox();
        //draw_rectangle_lines(pr.x, pr.y, pr.w, pr.h, 2.0, BLUE);

        if game_phase == GamePhase::Scene2Collision {
            draw_text_ex(
                "¡Choque!  Enter = Reiniciar",
                screen_width()*0.5 - 260.0,
                200.0,
                TextParams { font_size: 60, color: BLACK, ..Default::default() },
            );
        }
        next_frame().await;
    }
    game_phase
}