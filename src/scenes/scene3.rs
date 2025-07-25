
/// src/scenes/scene3.rs
use macroquad::{
    audio::{load_sound, play_sound, stop_sound, PlaySoundParams}, color::WHITE, input::KeyCode, texture::load_texture, window::{next_frame, screen_width}
};

use crate::{
    assets::{SCENE_3_AMBIENT, SCENE_3_REALIZE_1, SCENE_3_REALIZE_2, SCENE_3_REALIZE_3, SCENE_3_REALIZE_4, SCENE_3_REALIZE_5, SCENE_3_REALIZE_6, SCENE_3_REALIZE_7}, dialogue::{are_keys_pressed, draw_clean_fullscreen, zoom_over_time}, gui::text_box::{draw_text_background, TextBoxOpts}, types::GamePhase
};

pub async fn scene3() -> GamePhase {
    loop {
        draw_text_background(
                    "Begitxo sube al escenario y grita desesperado por que le ayuden a buscar sus gafas. (pulsa Enter para continuar)",
                    TextBoxOpts {
                        font_size: 50,
                        color: WHITE,
                        max_width: screen_width() * 0.8,
                        ..Default::default()
                    },
                )
                .await;

        if are_keys_pressed(&[KeyCode::Escape, KeyCode::Enter]).await {
            break;
        }
    }
    loop {
        draw_text_background(
            "La party responde y le piden que se fije en su camiseta. (pulsa Enter para continuar)",
            TextBoxOpts {
                font_size: 50,
                color: WHITE,
                max_width: screen_width() * 0.8,
                ..Default::default()
            },
        )
        .await;

        if are_keys_pressed(&[KeyCode::Escape, KeyCode::Enter]).await  {
            break;
        }
    }
    
    let begitxo_realize1 = load_texture(SCENE_3_REALIZE_1).await.unwrap();
    zoom_over_time(begitxo_realize1.clone(), 1.5, 2.0).await;

    let begitxo_realize2 = load_texture(SCENE_3_REALIZE_2).await.unwrap();
    zoom_over_time(begitxo_realize2.clone(), 1.5, 2.0).await;

    let begitxo_realize3 = load_texture(SCENE_3_REALIZE_3).await.unwrap();
    zoom_over_time(begitxo_realize3.clone(), 1.5, 2.0).await;

    let glasses_sound = load_sound(SCENE_3_AMBIENT).await.unwrap();
    play_sound(
        &glasses_sound,
        PlaySoundParams {
            looped: false,
            volume: 1.0,
        },
    );

    let begitxo_realize4 = load_texture(SCENE_3_REALIZE_4).await.unwrap();
    draw_clean_fullscreen(&begitxo_realize4.clone()).await;
    next_frame().await;

    std::thread::sleep(std::time::Duration::from_millis(200));
    let begitxo_realize5 = load_texture(SCENE_3_REALIZE_5).await.unwrap();
    draw_clean_fullscreen(&begitxo_realize5.clone()).await;
    next_frame().await;

    std::thread::sleep(std::time::Duration::from_millis(200));
    let begitxo_realize6 = load_texture(SCENE_3_REALIZE_6).await.unwrap();
    draw_clean_fullscreen(&begitxo_realize6.clone()).await;
    next_frame().await;

    std::thread::sleep(std::time::Duration::from_millis(200));
    let begitxo_realize7 = load_texture(SCENE_3_REALIZE_7).await.unwrap();
    draw_clean_fullscreen(&begitxo_realize7.clone()).await;
    next_frame().await;

    loop {
        draw_text_background(
            "Begitxo siempre tuvo sus gafas, más no pudo percatarse, tan ocupado se hallaba en la party.",
            TextBoxOpts {
                font_size: 50,
                color: WHITE,
                max_width: screen_width() * 0.8,
                ..Default::default()
            },
        )
        .await;

        if are_keys_pressed(&[KeyCode::Escape, KeyCode::Enter]).await  {
            break;
        }
    }

    loop {
        draw_text_background(
            "THE END",
            TextBoxOpts {
                font_size: 50,
                color: WHITE,
                max_width: screen_width() * 0.8,
                ..Default::default()
            },
        )
        .await;

        if are_keys_pressed(&[KeyCode::Escape, KeyCode::Enter]).await  {
            stop_sound(&glasses_sound);
            break;
        }
    }

    GamePhase::Scene3Win
}