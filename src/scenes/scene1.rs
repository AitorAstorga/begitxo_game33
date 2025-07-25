use crate::{
    assets::*,
    dialogue::zoom_over_time, gui::text_button::TextButton,
};
use macroquad::{
    audio::{
        load_sound, play_sound, play_sound_once, stop_sound, PlaySoundParams, Sound,
    }, color::WHITE, math::vec2, texture::{draw_texture, load_texture, Texture2D}, window::{clear_background, next_frame, screen_height, screen_width}
};

pub async fn scene1() {
    let music = load_sound(SCENE_1_MUSIC).await.unwrap();
    play_sound(
        &music,
        PlaySoundParams {
            looped: false,
            volume: 1.0,
        },
    );

    scene1_zoom_from_back().await;
    scene1_options(&music).await;

    stop_sound(&music);
}

/// Zoom from the back of Begitxo
async fn scene1_zoom_from_back() {
    let texture = load_texture(SCENE_1_BACK).await.unwrap();
    zoom_over_time(texture, 1.5, 3.0).await;
}

pub async fn scene1_options(music: &Sound) {
    let mut texture = load_texture(SCENE_1_BACK).await.unwrap();

    let font_size = 30;
    let pad = 12.0;

    let screen_w = screen_width();
    let screen_h = screen_height();
    let sleep_positions = [
        vec2(screen_w * 0.25, screen_h - 50.0),
        vec2(screen_w - 200.0, 40.0),
        vec2(screen_w * 0.5, screen_h - 50.0),
        vec2(40.0, screen_h - 50.0),
        vec2(screen_w - 200.0, 40.0),
    ];
    let mut dodge_idx = 0;

    let play_pos = vec2(screen_w * 0.75, screen_h - 50.0);
    let mut play_clicks = 0;

    loop {
        clear_background(WHITE);
        draw_texture(&texture, 0.0, 0.0, WHITE);

        let sleep_btn = TextButton::new("Ir a dormir", sleep_positions[dodge_idx], font_size);
        let (sleep_hovered, sleep_clicked) = sleep_btn.draw(pad);

        if sleep_hovered && !sleep_clicked && dodge_idx < sleep_positions.len() - 1 {
            dodge_idx += 1; // dodge next frame
        } else if sleep_clicked {
            scene1_electric_shock(music).await;
            break;
        }
        
        let play_btn = TextButton::new("Seguir jugando", play_pos, font_size);
        let (_, play_clicked) = play_btn.draw(pad);

        if play_clicked {
            play_clicks += 1;
            match play_clicks {
                1 => {
                    texture = scene1_irritated_eyes(SCENE_1_IRRITATED_1, 2.5).await;
                }
                2 => {
                    texture = scene1_irritated_eyes(SCENE_1_IRRITATED_2, 3.0).await;
                }
                3 => {
                    texture = scene1_irritated_eyes(SCENE_1_IRRITATED_3, 3.5).await;
                }
                4 => {
                    texture = scene1_irritated_eyes(SCENE_1_IRRITATED_4, 4.5).await;
                }
                5 => break, // finished the chain
                _ => (),
            }
        }

        next_frame().await;
    }
}

async fn scene1_irritated_eyes(path: &str, duration: f32) -> Texture2D {
    let texture = load_texture(path).await.unwrap();
    zoom_over_time(texture.clone(), 1.5, duration).await;
    texture
}

async fn scene1_electric_shock(music: &Sound) {
    stop_sound(music);

    let sound_shock = load_sound(SCENE_1_SHOCK).await.unwrap();
    play_sound_once(&sound_shock);

    let texture = load_texture(SCENE_3).await.unwrap();
    zoom_over_time(texture, 1.5, 2.0).await;
}