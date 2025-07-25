use macroquad::texture::load_texture;
use crate::dialogue::*;
use crate::assets::*;
use crate::types::GamePhase;

// Go to sleep or don't
pub async fn scene2() -> GamePhase {
    let texture_sun = load_texture(SCENE_2_SUN).await.unwrap();
    fade_in_texture(&texture_sun, 2.0).await;

    GamePhase::Win
}

/// Electric shock scene
pub async fn scene2_1() {

}