
// src/race/mod.rs
use macroquad::math::Vec2;
use crate::assets::_SCREEN_HEIGHT;

pub mod corridor;
pub mod obstacle;
pub mod player;

const GRAVITY: f32       = 2_000.0;
const JUMP_VELOCITY: f32 = -1250.0;
const GROUND_Y: f32      = _SCREEN_HEIGHT - 384.0;
const PLAYER_SIZE: Vec2  = Vec2::new(288.0, 384.0);
const HITBOX_SCALE: f32  = 2.0 / 3.0;