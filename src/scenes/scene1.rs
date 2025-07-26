use crate::{
    assets::*,
    dialogue::{draw_clean_fullscreen, shake_texture, zoom_over_time}, gui::text_button::TextButton, types::GamePhase,
};
use macroquad::{
    audio::{
        load_sound, play_sound, play_sound_once, stop_sound, PlaySoundParams, Sound,
    }, math::vec2, texture::{load_texture, Texture2D}, window::{next_frame, screen_height, screen_width}
};

pub async fn scene1() -> GamePhase {
    let music = load_sound(SCENE_1_MUSIC).await.unwrap();
    play_sound(
        &music,
        PlaySoundParams {
            looped: false,
            volume: 1.0,
        },
    );

    let ambient_sound = load_sound(SCENE_1_AMBIENT).await.unwrap();
    play_sound(
        &ambient_sound,
        PlaySoundParams {
            looped: true,
            volume: 0.5,
        },
    );

    scene1_zoom_from_back().await;
    let result = scene1_options(&music).await;

    stop_sound(&music);
    stop_sound(&ambient_sound);

    result
}

/// Zoom from the back of Begitxo
async fn scene1_zoom_from_back() {
    let texture = load_texture(SCENE_1_BACK).await.unwrap();
    zoom_over_time(texture, 1.5, 3.0).await;
}

/// Path to sleeping or staying awake
pub async fn scene1_options(music: &Sound) -> GamePhase {
    let mut texture = load_texture(SCENE_1_BACK).await.unwrap();

    let font_size = 30;

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
        draw_clean_fullscreen(&texture).await;

        let mut sleep_btn = TextButton::new("Ir a dormir", sleep_positions[dodge_idx], font_size);
        let (sleep_hovered, sleep_clicked) = sleep_btn.draw();

        if sleep_hovered && !sleep_clicked && dodge_idx < sleep_positions.len() - 1 {
            dodge_idx += 1; // dodge next frame
        } else if sleep_clicked {
            scene1_electric_shock(music).await;
            return GamePhase::Scene1Shock;
        }
        
        let mut play_btn = TextButton::new("Seguir jugando", play_pos, font_size);
        let (_, play_clicked) = play_btn.draw();

        if play_clicked {
            play_clicks += 1;
            match play_clicks {
                1 => {
                    texture = scene1_irritated_eye(SCENE_1_IRRITATED_1, 2.5).await;
                }
                2 => {
                    texture = scene1_irritated_eye(SCENE_1_IRRITATED_2, 3.0).await;
                }
                3 => {
                    texture = scene1_irritated_eye(SCENE_1_IRRITATED_3, 3.5).await;
                }
                4 => {
                    texture = scene1_irritated_eye(SCENE_1_IRRITATED_4, 4.5).await;
                }
                5 => return GamePhase::Scene1Awake, // finished the chain
                _ => (),
            }
        }

        next_frame().await;
    }
}

/// Helper to zoom in the irritated eye
async fn scene1_irritated_eye(path: &str, duration: f32) -> Texture2D {
    let texture = load_texture(path).await.unwrap();
    zoom_over_time(texture.clone(), 1.5, duration).await;
    texture
}

/// Electric shock
async fn scene1_electric_shock(music: &Sound) {
    stop_sound(music);

    let sound_shock = load_sound(SCENE_1_SHOCK_SOUND).await.unwrap();
    play_sound_once(&sound_shock);

    let texture1 = load_texture(SCENE_1_SHOCK_1).await.unwrap();
    zoom_over_time(texture1, 1.5, 2.0).await;

    let texture2 = load_texture(SCENE_1_SHOCK_2).await.unwrap();
    shake_texture(&texture2, 1.5, 20.0, 2.0).await;
}