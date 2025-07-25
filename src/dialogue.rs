// src/dialogue.rs
use macroquad::prelude::*;

pub async fn draw_clean_texture(texture: Texture2D) {
    clear_background(BLACK);
    draw_texture(&texture, 0.0, 0.0, WHITE);
    next_frame().await;
}

/// Waits for a key to be pressed.
async fn wait_for_key_press(texture: Texture2D, key: KeyCode) {
    loop {
        draw_clean_texture(texture.clone()).await;
        if is_key_pressed(key) {
            break;
        }
    }
}

/// Advances slides automatically after a fixed `interval` (in seconds).
async fn auto_advance(texture: Texture2D, interval: f32) {
    let start = get_time();
    loop {
        let elapsed = (get_time() - start) as f32;
        if elapsed >= interval {
            break;
        }
        draw_clean_texture(texture.clone()).await;
    }
}

/// Displays a series of images and waits for a key to be pressed in each.
pub async fn show_slides_on_key(images: &[&str], key: KeyCode) {
    for image_path in images.iter() {
        let texture = load_texture(image_path).await.unwrap();
        wait_for_key_press(texture, key).await;
        // Texture is dropped here before loading the next image.
    }
}

/// Displays a series of images, advancing automatically every `interval` seconds.
pub async fn show_slides_auto(images: &[&str], interval: f32) {
    for &path in images.iter() {
        let texture = load_texture(path).await.unwrap();
        auto_advance(texture, interval).await;
    }
}

/// Zooms the given `texture` to `target_scale` over `duration` seconds.
/// `target_scale` is a multiplier (e.g., 1.5 = 150% zoom).
pub async fn zoom_over_time(texture: Texture2D, target_scale: f32, duration: f32) {
    let start_time = get_time();
    let initial_scale = 1.0;
    let (screen_w, screen_h) = (screen_width(), screen_height());
    let (tex_w, tex_h) = (texture.width(), texture.height());

    loop {
        let t = (get_time() - start_time) as f32;
        let progress = (t / duration).min(1.0);
        let scale = initial_scale + (target_scale - initial_scale) * progress;

        let draw_w = tex_w * scale;
        let draw_h = tex_h * scale;
        let x = (screen_w - draw_w) / 2.0;
        let y = (screen_h - draw_h) / 2.0;

        clear_background(BLACK);
        draw_texture_ex(
            &texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(draw_w, draw_h)),
                ..Default::default()
            },
        );
        next_frame().await;

        if progress >= 1.0 {
            break;
        }
    }
}

pub async fn fade_in_texture(texture: &Texture2D, secs: f32) {
    let mut elapsed = 0.0;

    while elapsed < secs {
        // Interpolate α from 0.0 ➜ 1.0
        let alpha = (elapsed / secs).clamp(0.0, 1.0);

        clear_background(BLACK);

        // Draw centred, full‑screen (adjust if you need another layout)
        let (w, h) = (screen_width(), screen_height());
        macroquad::texture::draw_texture_ex(
            texture,
            0.0,
            0.0,
            Color::new(1.0, 1.0, 1.0, alpha),
            macroquad::texture::DrawTextureParams {
                dest_size: Some(vec2(w, h)),
                ..Default::default()
            },
        );

        next_frame().await;
        elapsed += get_frame_time();
    }

    // Ensure final opaque frame is shown
    clear_background(BLACK);
    macroquad::texture::draw_texture_ex(
        texture,
        0.0,
        0.0,
        WHITE,
        macroquad::texture::DrawTextureParams {
            dest_size: Some(vec2(screen_width(), screen_height())),
            ..Default::default()
        },
    );
    next_frame().await;
}