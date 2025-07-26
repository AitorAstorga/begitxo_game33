// src/race/obstacle.rs
use crate::race::{corridor::ScrollDir, GROUND_Y, PLAYER_SIZE};
use macroquad::{
    color::WHITE,
    math::{Rect, Vec2},
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
    window::screen_width,
};

pub struct Obstacle {
    x: f32,
    tex: Texture2D,
    size: Vec2,
}

impl Obstacle {
    pub fn new(x: f32, tex: Texture2D, size: Vec2) -> Self {
        Self { x, tex, size }
    }

    pub fn update(&mut self, speed: f32, dir: ScrollDir, dt: f32) {
        self.x += match dir {
            ScrollDir::Left => -speed,
            ScrollDir::Right => speed,
        } * dt;
    }

    pub fn rect(&self) -> Rect {
        Rect::new(
            self.x - 3.0,
            GROUND_Y + PLAYER_SIZE.y - self.size.y,
            self.size.x,
            self.size.y,
        )
    }

    pub fn draw(&self) {
        draw_texture_ex(
            &self.tex,
            self.x,
            GROUND_Y + PLAYER_SIZE.y - self.size.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.size),
                ..Default::default()
            },
        );
    }

    pub fn offscreen(&self, dir: ScrollDir) -> bool {
        match dir {
            ScrollDir::Left => self.x + self.size.x <= -64.0,
            ScrollDir::Right => self.x >= screen_width() + 64.0,
        }
    }
}