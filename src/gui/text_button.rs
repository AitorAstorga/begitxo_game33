use macroquad::{color::{BLACK, GRAY, LIGHTGRAY}, input::{is_mouse_button_pressed, mouse_position, MouseButton}, math::{Rect, Vec2}, shapes::draw_rectangle, text::{draw_text, measure_text, TextDimensions}};

// src/gui/text_button.rs
pub struct TextButton {
    label: &'static str,
    pos: Vec2,
    dim: TextDimensions,
    font_size: u16,
}

impl TextButton {
    pub fn new(label: &'static str, pos: Vec2, font_size: u16) -> Self {
        let dim = measure_text(label, None, font_size, 1.0);
        Self {
            label,
            pos,
            dim,
            font_size,
        }
    }

    pub fn rect(&self, padding: f32) -> Rect {
        Rect::new(
            self.pos.x - padding,
            self.pos.y - self.dim.height - padding,
            self.dim.width + padding * 2.0,
            self.dim.height + padding * 2.0,
        )
    }

    /// Draw the button.  Returns `(hovered, clicked_this_frame)`.
    pub fn draw(&self, padding: f32) -> (bool, bool) {
        let mouse = mouse_position();
        let hovered = self
            .rect(padding)
            .contains(Vec2::new(mouse.0, mouse.1));

        // Background
        let bg_col = if hovered { GRAY } else { LIGHTGRAY };
        draw_rectangle(
            self.rect(padding).x,
            self.rect(padding).y,
            self.rect(padding).w,
            self.rect(padding).h,
            bg_col,
        );

        // Text (centre‑left aligned like normal macroquad text)
        draw_text(
            self.label,
            self.pos.x,
            self.pos.y + self.dim.height,
            self.font_size as f32,
            BLACK,
        );

        let clicked = hovered && is_mouse_button_pressed(MouseButton::Left);
        (hovered, clicked)
    }
}