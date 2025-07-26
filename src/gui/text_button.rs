use macroquad::{
    color::{Color, BLACK, DARKGRAY, GRAY, WHITE},
    input::{is_mouse_button_pressed, mouse_position, MouseButton},
    math::{Rect, Vec2},
    shapes::draw_rectangle,
    text::{draw_text, measure_text, TextDimensions},
};

/// Visual tweaks bundled together so code that creates a button
/// can decide colours, padding, etc. without touching the widget code.
#[derive(Clone)]
pub struct ButtonStyle {
    pub padding: f32,
    pub text_color: Color,
    pub bg: Color,
    pub bg_hover: Color,
    pub bg_pressed: Color,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            padding: 6.0,
            text_color: BLACK,
            bg: WHITE,
            bg_hover: GRAY,
            bg_pressed: DARKGRAY,
        }
    }
}

/// A simple text‑only button. `top_left` really *is* the button's top‑left corner.
pub struct TextButton {
    label: String,
    top_left: Vec2,
    font_size: u16,
    style: ButtonStyle,
    dims: TextDimensions,
}

impl TextButton {
    pub fn new<S: Into<String>>(label: S, top_left: Vec2, font_size: u16) -> Self {
        let label = label.into();
        let dims = measure_text(&label, None, font_size, 1.0);
        Self {
            label,
            top_left,
            font_size,
            style: ButtonStyle::default(),
            dims,
        }
    }

    /// Rectangle that encloses *everything* (background + padding).
    pub fn rect(&self) -> Rect {
        Rect::new(
            self.top_left.x,
            self.top_left.y,
            self.dims.width + self.style.padding * 2.0,
            self.dims.height + self.style.padding * 2.0,
        )
    }

    /// Draw and return (hovered, clicked_this_frame).
    pub fn draw(&mut self) -> (bool, bool) {
        let current_dims = measure_text(&self.label, None, self.font_size, 1.0);
        if current_dims.width != self.dims.width {
            self.dims = current_dims;
        }

        let rect = self.rect();
        let (mx, my) = mouse_position();
        let hovered = rect.contains(Vec2::new(mx, my));
        let pressed = hovered && is_mouse_button_pressed(MouseButton::Left);

        // -----------------------------------------------------------------------
        // Background
        // -----------------------------------------------------------------------
        let bg = if pressed {
            self.style.bg_pressed
        } else if hovered {
            self.style.bg_hover
        } else {
            self.style.bg
        };
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, bg);

        // -----------------------------------------------------------------------
        // Label
        // -----------------------------------------------------------------------
        let text_x = rect.x + (rect.w - self.dims.width) * 0.5;
        // vertical centre: top_of_rect  +   half_free_space + offset_y
        // (`offset_y` is “distance from baseline to the *top* of the glyphs”)
        // see docs.rs example for the maths :contentReference[oaicite:0]{index=0}
        let text_baseline = rect.y
            + (rect.h - self.dims.height) * 0.5   // halfway down the empty band
            + self.dims.offset_y;                 // then down to the baseline

        draw_text(
            &self.label,
            text_x,
            text_baseline,
            self.font_size as f32,
            self.style.text_color,
        );

        (hovered, pressed)
    }
}