// src/types.rs

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GamePhase {
    scene1_shock,
    scene1_awake,
    scene2_dialogue,
    scene2_race,
    scene2_collision,
    Win,
}
