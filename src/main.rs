// src/main.rs
mod gui;
mod types;
mod assets;
mod dialogue;
mod game;
mod scenes;
use game::run_game;

#[macroquad::main("Fallen God")]
async fn main() {
    run_game().await;
}
