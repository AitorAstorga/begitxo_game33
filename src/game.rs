// src/game.rs
use macroquad::{color::WHITE, input::KeyCode, window::screen_width};
use crate::{dialogue::are_keys_pressed, gui::text_box::{draw_text_background, TextBoxOpts}, scenes::{scene1::scene1, scene2::scene2, scene3::scene3}, types::GamePhase};

pub async fn run_game() {
    let mut game_phase: GamePhase;
    loop {                
        // Scene 1
        game_phase = scene1().await;

        if game_phase == GamePhase::Scene1Shock {
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

                if are_keys_pressed(&[KeyCode::Escape, KeyCode::Enter]).await  {
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

            if are_keys_pressed(&[KeyCode::Escape, KeyCode::Enter]).await  {
                break;
            }
        }

        // Scene 2
        game_phase = scene2().await;


        if game_phase == GamePhase::Scene2Race {
            game_phase = scene3().await;
        }

        
        
    }
}