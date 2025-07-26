// src/gui/text_box.rs

use macroquad::{color::{Color, BLACK, WHITE}, math::{vec2, Vec2}, text::{draw_text_ex, measure_text, TextParams}, window::{clear_background, next_frame, screen_height, screen_width}};

/// Tweaks for drawing a paragraph of text.
#[derive(Clone)]
pub struct TextBoxOpts {
    /// Font size in logical pixels.
    pub font_size: u16,
    /// Text colour.
    pub color: Color,
    /// Maximum line width before wrapping (px).  
    /// Tip: `screen_width() * 0.9` ⇒ ~90 % of the window.
    pub max_width: f32,
    /// Top‑left corner of the *bounding* rectangle (ignored if *both*
    /// `h_center` and `v_center` are `true`).
    pub pos: Vec2,
    /// Horizontally centre *every* line inside the window?
    pub h_center: bool,
    /// Vertically centre the whole paragraph?
    pub v_center: bool,
}

impl Default for TextBoxOpts {
    fn default() -> Self {
        Self {
            font_size: 32,
            color: WHITE,
            max_width: screen_width() * 0.9,
            pos: vec2(0.0, 0.0),
            h_center: true,
            v_center: true,
        }
    }
}

/// Draw a black background and render `text` according to `opts`.  
/// One frame is produced, but the `async` keeps the same call style as
/// your other helpers.
pub async fn draw_text_background(text: &str, opts: TextBoxOpts) {
    // -----------------------------------------------------------------------
    // Word‑wrap ----------------------------------------------------------------
    // -----------------------------------------------------------------------
    let mut lines: Vec<String> = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        // Attempt to add `word` to the current line …
        let tentative = if current.is_empty() {
            word.to_string()
        } else {
            format!("{current} {word}")
        };

        let text_dimensions = measure_text(&tentative, None, opts.font_size, 1.0);

        if text_dimensions.width > opts.max_width && !current.is_empty() {
            // Too wide: push the line we *did* fit, start anew.
            lines.push(current);
            current = word.to_string();
        } else {
            current = tentative;
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }

    // -----------------------------------------------------------------------
    // Positioning / centring -------------------------------------------------
    // -----------------------------------------------------------------------
    let line_height = opts.font_size as f32 * 1.25;
    let para_height = line_height * lines.len() as f32;

    let base_y = if opts.v_center {
        (screen_height() - para_height) * 0.5
    } else {
        opts.pos.y
    };

    // -----------------------------------------------------------------------
    // Draw -------------------------------------------------------------------
    // -----------------------------------------------------------------------
    clear_background(BLACK);

    for (i, line) in lines.iter().enumerate() {
        let text_dimensions = measure_text(line, None, opts.font_size, 1.0);

        let x = if opts.h_center {
            (screen_width() - text_dimensions.width) * 0.5
        } else {
            opts.pos.x
        };

        let y = base_y + line_height * i as f32 + opts.font_size as f32; // baseline

        draw_text_ex(
            line,
            x,
            y,
            TextParams {
                font_size: opts.font_size,
                color: opts.color,
                ..Default::default()
            },
        );
    }

    next_frame().await;
}