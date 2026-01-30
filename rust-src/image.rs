use ab_glyph::{FontRef, PxScale};
use image::{Rgb, RgbImage};
use imageproc::drawing::draw_text_mut;

pub const DISPLAY_WIDTH: u32 = 160;
pub const DISPLAY_HEIGHT: u32 = 80;

const FONT_DATA: &[u8] = include_bytes!("../assets/fonts/DejaVuSans.ttf");

pub fn create_blank_image() -> RgbImage {
    RgbImage::from_pixel(DISPLAY_WIDTH, DISPLAY_HEIGHT, Rgb([0, 0, 0]))
}

pub fn create_text_image(text: &str, font_size: f32) -> RgbImage {
    let mut img = create_blank_image();
    draw_text(&mut img, text, font_size);
    img
}

fn draw_text(img: &mut RgbImage, text: &str, font_size: f32) {
    let font = FontRef::try_from_slice(FONT_DATA).expect("Failed to load embedded font");
    let scale = PxScale::from(font_size);

    let (text_width, text_height) = measure_text(&font, scale, text);

    let x = ((DISPLAY_WIDTH as i32 - text_width as i32) / 2).max(0);
    let y = ((DISPLAY_HEIGHT as i32 - text_height as i32) / 2).max(0);

    draw_text_mut(img, Rgb([255, 255, 255]), x, y, scale, &font, text);
}

fn measure_text(font: &FontRef, scale: PxScale, text: &str) -> (u32, u32) {
    use ab_glyph::{Font, ScaleFont};

    let scaled_font = font.as_scaled(scale);
    let mut width = 0.0f32;
    let height = scaled_font.height();

    for c in text.chars() {
        let glyph_id = font.glyph_id(c);
        width += scaled_font.h_advance(glyph_id);
    }

    (width as u32, height as u32)
}

pub fn image_to_rgb565_bytes(img: &RgbImage) -> Vec<u8> {
    let mut data = Vec::with_capacity((DISPLAY_WIDTH * DISPLAY_HEIGHT * 2) as usize);

    for y in 0..DISPLAY_HEIGHT {
        for x in 0..DISPLAY_WIDTH {
            let pixel = img.get_pixel(x, y);
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];

            let r5 = (r >> 3) & 0x1F;
            let g6 = (g >> 2) & 0x3F;
            let b5 = (b >> 3) & 0x1F;

            let rgb565 = ((r5 as u16) << 11) | ((g6 as u16) << 5) | (b5 as u16);

            data.push((rgb565 & 0xFF) as u8);
            data.push((rgb565 >> 8) as u8);
        }
    }

    data
}
