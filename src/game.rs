// src/game.rs
use crate::{scenes::{scene1::{scene1}, scene2::scene2}, types::GamePhase};

pub async fn run_game() {    
    loop {        
        let mut state;
        
        // Scene 1
        scene1().await;

        // Scene 2
        loop {
            state = scene2().await;
        }
        
    }
}
