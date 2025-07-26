// src/dialogue.rs
use macroquad::{prelude::*, rand::rand};

/// ---------------------------------------------------------------------------
/// Small helpers
/// ---------------------------------------------------------------------------

/// Draw `texture` so it completely fills the window.
fn draw_fullscreen(texture: &Texture2D, offset: Vec2, tint: Color) {
    let dest_size = vec2(screen_width(), screen_height());
    draw_texture_ex(
        texture,
        offset.x,
        offset.y,
        tint,
        DrawTextureParams {
            dest_size: Some(dest_size),
            ..Default::default()
        },
    );
}

/// Draw the texture at its original size in the top‑left corner.
pub async fn draw_clean_fullscreen(texture: &Texture2D) {
    clear_background(BLACK);
    draw_fullscreen(texture, Vec2::ZERO, WHITE);
    next_frame().await;
}

/// Draw an empty background with a text and the given colors and dimensions.
async fn draw_text_background(text: &str, color: Color, pos: Vec2, font_size: f32) {
    clear_background(BLACK);
    draw_text(text, pos.x, pos.y, font_size, color);
    next_frame().await;
}

/// ---------------------------------------------------------------------------
/// Effects
/// ---------------------------------------------------------------------------

/// Zoom from “fill‑screen” to `target_scale ×` bigger over `duration` seconds.
///
/// `target_scale = 1.25` -> 25% bigger than the starting size.
pub async fn zoom_over_time(texture: Texture2D, target_scale: f32, duration: f32) {
    let base_scale = {
        let sx = screen_width() / texture.width();
        let sy = screen_height() / texture.height();
        sx.max(sy)
    };

    let start = get_time();

    loop {
        let t = ((get_time() - start) as f32 / duration).clamp(0.0, 1.0);

        let scale = base_scale * (1.0 + (target_scale - 1.0) * t);
        let size  = vec2(texture.width() * scale, texture.height() * scale);
        let pos   = vec2((screen_width()  - size.x) * 0.5,
                         (screen_height() - size.y) * 0.5);

        clear_background(BLACK);
        draw_texture_ex(
            &texture,
            pos.x,
            pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(size),
                ..Default::default()
            },
        );
        next_frame().await;

        if t >= 1.0 { break; }
    }
}

/// Fade the picture in over `secs` seconds.
pub async fn fade_in_texture(texture: &Texture2D, secs: f32) {
    let mut elapsed = 0.0;

    while elapsed < secs {
        let alpha = (elapsed / secs).clamp(0.0, 1.0);
        clear_background(BLACK);
        draw_fullscreen(texture, Vec2::ZERO, Color::new(1.0, 1.0, 1.0, alpha));
        next_frame().await;
        elapsed += get_frame_time();
    }

    draw_clean_fullscreen(texture).await;
}

/// Shake the whole screen for `secs` seconds.
///
/// * `amount` – maximum offset in **pixels**.
/// * `speed`  – shakes per second (frequency).
pub async fn shake_texture(texture: &Texture2D, secs: f32, amount: f32, speed: f32) {
    let mut elapsed = 0.0;

    while elapsed < secs {
        let rng = rand() as f32;
        let angle  = elapsed * speed * rng;
        let offset = vec2(angle.sin() * amount, angle.cos() * amount);

        clear_background(BLACK);
        draw_fullscreen(texture, offset, WHITE);
        next_frame().await;

        elapsed += get_frame_time();
    }

    // Recenter the texture
    draw_clean_fullscreen(texture).await;
}
