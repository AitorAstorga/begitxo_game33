use macroquad::input::KeyCode;
use crate::dialogue::*;
use crate::assets::*;
use crate::types::GamePhase;

// Go to sleep or don't
pub async fn scene2() -> GamePhase {
    // TO DO: {IMAGEN ZOOM-IN LENTO DE BEGITXO JUGANDO DE ESPALDAS}
    let scene1_images = [
        SCENE_1_1,
        SCENE_1_2,
        SCENE_1_3,
        SCENE_1_4,
    ];
    // TO DO: Play menu music
    // - {SONIDO DE STOCK DE RUIDO PC}
    // - {SONIDO DE CLICKS}
    show_slides_on_key(&scene1_images, KeyCode::Space).await;
    GamePhase::Win
}

/// Electric shock scene
pub async fn scene2_1() {

}