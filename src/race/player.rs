// src/race/player.rs
use macroquad::{
    color::WHITE,
    math::Rect,
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
};

use crate::race::{GRAVITY, GROUND_Y, HITBOX_SCALE, JUMP_VELOCITY, PLAYER_SIZE, PLAYER_X};

pub struct Player {
    tex: Texture2D,
    y: f32,
    vy: f32,
}

impl Player {
    pub fn new(tex: Texture2D) -> Self {
        Self {
            tex,
            y: GROUND_Y,
            vy: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.vy += GRAVITY * dt;
        self.y += self.vy * dt;
        if self.y > GROUND_Y {
            self.y = GROUND_Y;
            self.vy = 0.0;
        }
    }

    pub fn jump(&mut self) {
        if self.y >= GROUND_Y - 0.5 {
            self.vy = JUMP_VELOCITY;
        }
    }

    pub fn hitbox(&self) -> Rect {
        let w = PLAYER_SIZE.x * HITBOX_SCALE - 12.0;
        let h = PLAYER_SIZE.y - 3.0;
        let dx = (PLAYER_SIZE.x - w) * 0.5;
        let dy = (PLAYER_SIZE.y - h) * 0.5;
        Rect::new(PLAYER_X + dx, self.y + dy, w, h,)
    }

    pub fn draw(&self) {
        draw_texture_ex(
            &self.tex,
            PLAYER_X,
            self.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(PLAYER_SIZE),
                ..Default::default()
            },
        );
    }

    pub fn reset(&mut self) {
        self.y = GROUND_Y;
        self.vy = 0.0;
    }
}
