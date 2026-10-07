use image::{DynamicImage, ImageFormat};
use rusttype::{point, Font, Scale};
use std::io::Cursor;
use wasm_minimal_protocol::*;

initiate_protocol!();

static FONT_DATA: &[u8] = include_bytes!("../fonts/font.ttf");

#[wasm_func]
pub fn bake_watermark(
    image_bytes: &[u8],
    text_bytes: &[u8],
    opacity_bytes: &[u8],
) -> Result<Vec<u8>, String> {
    let text = std::str::from_utf8(text_bytes).map_err(|e| format!("bad text: {e}"))?;
    let mut img = image::load_from_memory(image_bytes)
        .map_err(|e| format!("cannot decode image: {e}"))?
        .to_rgba8();
    let img_width = img.width();
    let img_height = img.height();
    let w = img_width as f32;
    let h = img_height as f32;

    let font = Font::try_from_bytes(FONT_DATA).ok_or_else(|| format!("cannot load font: FONT_DATA_LEN={}", FONT_DATA.len()))?;
    let alpha_mul = opacity_bytes.first().copied().unwrap_or(128) as f32 / 255.0;

    // Measure text at a reference size, then scale so it spans ~60% of image width
    let text_width = |size: f32| -> f32 {
        let scale = Scale::uniform(size);
        let v = font.v_metrics(scale);
        font.layout(text, scale, point(0.0, v.ascent))
            .last()
            .map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
            .unwrap_or(0.0)
    };
    let ref_w = text_width(100.0);
    if ref_w <= 0.0 {
        return Err("empty watermark text".into());
    }
    let size = (100.0 * (w * 0.6) / ref_w).min(h * 0.5);

    let scale = Scale::uniform(size);
    let v = font.v_metrics(scale);
    let x0 = (w - text_width(size)) / 2.0;
    let y0 = (h - (v.ascent - v.descent)) / 2.0;

    for glyph in font.layout(text, scale, point(x0, y0 + v.ascent)) {
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|gx, gy, coverage| {
                let x = bb.min.x + gx as i32;
                let y = bb.min.y + gy as i32;
                if x < 0 || y < 0 {
                    return;
                }
                let x_u = x as u32;
                let y_u = y as u32;
                if x_u >= img_width || y_u >= img_height {
                    return;
                }
                let a = coverage * alpha_mul;
                let p = img.get_pixel_mut(x_u, y_u);
                for c in 0..3 {
                    p[c] = (p[c] as f32 * (1.0 - a) + 255.0 * a).round() as u8; // white text
                }
                p[3] = p[3].max((a * 255.0) as u8);
            });
        }
    }

    let mut out = Vec::new();
    DynamicImage::ImageRgba8(img)
        .write_to(&mut Cursor::new(&mut out), ImageFormat::Png)
        .map_err(|e| format!("cannot encode png: {e}"))?;
    Ok(out)
}
