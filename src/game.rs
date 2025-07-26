// src/game.rs
use macroquad::{color::WHITE, input::{is_key_pressed, KeyCode}, window::screen_width};
use crate::{gui::text_box::{draw_text_background, TextBoxOpts}, scenes::{scene1::scene1, scene2::scene2}, types::GamePhase};

pub async fn run_game() {
    let mut game_phase: GamePhase;
    loop {                
        // Scene 1
        game_phase = scene1().await;

        if game_phase == GamePhase::scene1_shock {
            loop {
                draw_text_background(
                    "Begitxo intenta desconectar su puesto antes de dormir. Descansa en paz, Begitxo. (pulsa Escape para salir)",
                    TextBoxOpts {
                        font_size: 50,
                        color: WHITE,
                        max_width: screen_width() * 0.8,
                        ..Default::default()
                    },
                )
                .await;

                if is_key_pressed(KeyCode::Escape) {
                    break;
                }
            }
            break;
        }

        // Intermediary scene: show it until Enter is pressed
        loop {
            draw_text_background(
                "Begitxo se queda hasta que llega el Sol al amanecer (pulsa Enter para continuar)",
                //"Begitxo stays until the sun comes up in the morning (press Enter to continue)",
                TextBoxOpts {
                    font_size: 50,
                    color: WHITE,
                    max_width: screen_width() * 0.8,
                    ..Default::default()
                },
            )
            .await;

            if is_key_pressed(KeyCode::Enter) {
                break;
            }
        }

        // Scene 2
        game_phase = scene2().await;

        if game_phase == GamePhase::scene2_race {
            // TO DO: All this code to scene3
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

                if is_key_pressed(KeyCode::Enter) {
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

                if is_key_pressed(KeyCode::Enter) {
                    break;
                }
            }
            // TO DO: Create begitxo in escenario to be zoomed in
            //let begitxo = load_texture(SCENE3_BEGITXO).await.unwrap();
            // zoom_over_time(texture.clone(), 1.5, duration).await;
        }
        
    }
}