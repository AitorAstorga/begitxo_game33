use macroquad::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ScrollDir { Left, Right }

pub struct CorridorScroller {
    tex: Texture2D,
    x:   f32,
    pub speed: f32,
    w:   f32,
    pub(crate) dir: ScrollDir,
    loops_goal: usize,   // how many full tiles to travel
    loops_done: usize,
}

impl CorridorScroller {
    pub fn from_texture(tex: Texture2D, speed: f32, dir: ScrollDir, tiles: usize) -> Self {
        assert!(speed >= 0.0 && tiles >= 1);
        tex.set_filter(FilterMode::Nearest);
        Self { w: tex.width(), tex, x: 0.0, speed, dir, loops_goal: tiles, loops_done: 0 }
    }

    #[inline] pub fn update(&mut self) {
        let dt = get_frame_time();
        self.x += match self.dir { ScrollDir::Left => -self.speed, ScrollDir::Right => self.speed } * dt;
        if self.x <= -self.w { self.x += self.w; self.loops_done += 1; }
        else if self.x >=  self.w { self.x -= self.w; self.loops_done += 1; }
    }

    #[inline] pub fn finished(&self) -> bool { self.loops_done >= self.loops_goal }

    #[inline] pub fn draw(&self) {
        draw_texture(&self.tex, self.x, 0.0, WHITE);
        let other_x = if self.dir == ScrollDir::Left { self.x + self.w } else { self.x - self.w };
        draw_texture(&self.tex, other_x, 0.0, WHITE);
    }
}