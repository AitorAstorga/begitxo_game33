// src/assets.rs

pub struct Image {
    pub path: &'static str,
    pub width: f32,
    pub height: f32,
}

// ==========================
// Dimensions
// ==========================
pub const SCREEN_WIDTH: f32 = 640.0;
pub const SCREEN_HEIGHT: f32 = 480.0;

// ==========================
// Maps and Backgrounds
// ==========================
pub const SCENE_1_BACK: &str = "./img/begitxo_back.png";
pub const SCENE_1_IRRITATED_1: &str = "./img/begitxo_irritated1.png";
pub const SCENE_1_IRRITATED_2: &str = "./img/begitxo_irritated2.png";
pub const SCENE_1_IRRITATED_3: &str = "./img/begitxo_irritated3.png";
pub const SCENE_1_IRRITATED_4: &str = "./img/begitxo_irritated4.png";

pub const SCENE_1_SHOCK_1: &str = "./img/begitxo_shock1.png";
pub const SCENE_1_SHOCK_2: &str = "./img/begitxo_shock2.png";

pub const SCENE_2_SUN: &str = "./img/begitxo_sun.png";

pub const SCENE_2_RACE: &str = "./img/race/corridor.png";

// ==========================
// Player Images
// ==========================
pub const SCENE_2_PLAYER: &str = "./img/begitxo_player.png";

// ==========================
// Obstacles
// ==========================
pub const SCENE_2_OBSTACLE: &str = "./img/missing_texture_TEST_35x50.png";
pub const SCENE_2_OBSTACLE_1: &str = "./img/race/obstacle1.png";
pub const SCENE_2_OBSTACLE_2: &str = "./img/race/obstacle2.png";


// ==========================
// Sounds
// ==========================

pub const SCENE_1_MUSIC: &str = "./sound/scene1.ogg";
pub const SCENE_1_AMBIENT: &str = "./sound/scene1.ogg"; // TO DO: ERROR WITH MP3 IN SOME SYSTEMS BECAUSE YES "./sound/scene1_ambient.mp3";
pub const SCENE_1_SHOCK_SOUND: &str = "./sound/scene1_electric_shock.ogg";

// ==========================
// Menu and Other UI Images
// ==========================
