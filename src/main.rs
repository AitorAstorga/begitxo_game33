// src/main.rs
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
