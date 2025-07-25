// src/game.rs
use crate::{scenes::{scene1::{scene1}, scene2::scene2}, types::GamePhase};

pub async fn run_game() {    
    loop {        
        //let mut state;
        
        // Scene 1
        scene1().await;

        // Intermediary scene
        // Begitxo stays until the sun comes up in the morning

        // Scene 2
        scene2().await;
        
    }
}
