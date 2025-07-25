use crate::{assets::*, dialogue::{draw_clean_texture, zoom_over_time}};
use macroquad::{audio::{load_sound, play_sound, play_sound_once, stop_sound, PlaySoundParams, Sound}, color::{BLACK, WHITE}, input::{is_mouse_button_pressed, mouse_position, MouseButton}, math::{vec2, Rect, Vec2}, text::{draw_text, measure_text}, texture::{draw_texture, load_texture}, window::{clear_background, next_frame, screen_height, screen_width}};

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
    let texture = load_texture(SCENE_1_1).await.unwrap();
    zoom_over_time(texture, 1.5, 3.0).await;
}

pub async fn scene1_options(music: &Sound) {
    let texture = load_texture(SCENE_1_1).await.unwrap();
    // Precompute text dimensions
    let sleep_text = "Ir a dormir";
    let play_text = "Seguir jugando";
    let font_size = 30;
    let sleep_dim = measure_text(sleep_text, None, font_size as u16, 1.0);
    let play_dim = measure_text(play_text, None, font_size as u16, 1.0);

    // Positions for sleep button dodging sequence
    let screen_w = screen_width();
    let screen_h = screen_height();
    let padding = 20.0;
    let sleep_positions = [
        // Start: bottom-center left
        vec2(screen_w * 0.25 - sleep_dim.width / 2.0, screen_h - sleep_dim.height - padding),
        // Top-right corner
        vec2(screen_w - sleep_dim.width - padding, padding),
        // Bottom-center
        vec2(screen_w * 0.5 - sleep_dim.width / 2.0, screen_h - sleep_dim.height - padding),
        // Bottom-left corner
        vec2(padding, screen_h - sleep_dim.height - padding),
        // Top-right again before final click
        vec2(screen_w - sleep_dim.width - padding, padding),
    ];
    let mut dodge_index = 0;

    // Static play button position: bottom-center right
    let play_pos = vec2(screen_w * 0.75 - play_dim.width / 2.0, screen_h - play_dim.height - padding);

    loop {
        clear_background(WHITE);
        draw_texture(&texture, 0.0, 0.0, WHITE);

        // Current rects
        let sleep_pos = sleep_positions[dodge_index];
        let sleep_rect = Rect::new(sleep_pos.x, sleep_pos.y, sleep_dim.width, sleep_dim.height);
        let play_rect = Rect::new(play_pos.x, play_pos.y, play_dim.width, play_dim.height);

        // Draw buttons
        draw_text(sleep_text, sleep_pos.x, sleep_pos.y + sleep_dim.height, font_size as f32, BLACK);
        draw_text(play_text, play_pos.x, play_pos.y + play_dim.height, font_size as f32, BLACK);

        let mouse = mouse_position();
        let hovered_sleep = sleep_rect.contains(Vec2::new(mouse.0, mouse.1));
        let hovered_play = play_rect.contains(Vec2::new(mouse.0, mouse.1));

        // Sleep-hover logic: move on hover until last state
        if hovered_sleep {
            if dodge_index < sleep_positions.len() - 1 {
                dodge_index += 1;
            } else if is_mouse_button_pressed(MouseButton::Left) {
                scene1_electric_shock(&music).await;
                break;
            }
        }

        // Play-click logic: advance through scene chain
        if hovered_play && is_mouse_button_pressed(MouseButton::Left) {
            //scene4().await;
            break;
        }

        next_frame().await;
    }
}

async fn scene1_electric_shock(music: &Sound) {
    stop_sound(music);

    let sound_shock = load_sound(SCENE_1_SHOCK).await.unwrap();
    play_sound_once(&sound_shock);

    let texture = load_texture(SCENE_3).await.unwrap();
    zoom_over_time(texture, 1.5, 2.0).await;

    stop_sound(&sound_shock);
}