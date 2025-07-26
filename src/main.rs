// src/main.rs
mod gui;
mod race;
mod types;
mod assets;
mod dialogue;
mod game;
mod scenes;
use game::run_game;

#[macroquad::main("Begitxo Game 33")]
async fn main() {
    run_game().await;
}
