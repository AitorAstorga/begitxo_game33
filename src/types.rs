// src/types.rs

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GamePhase {
    Scene1Shock,
    Scene1Awake,
    Scene2Dialogue,
    Scene2Race,
    Scene2Collision,
    Scene3Win,
}
