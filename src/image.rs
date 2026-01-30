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

#[cfg(test)]
fn rgb_to_rgb565(r: u8, g: u8, b: u8) -> u16 {
    let r5 = (r >> 3) & 0x1F;
    let g6 = (g >> 2) & 0x3F;
    let b5 = (b >> 3) & 0x1F;
    ((r5 as u16) << 11) | ((g6 as u16) << 5) | (b5 as u16)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_blank_image_dimensions() {
        let img = create_blank_image();
        assert_eq!(img.width(), DISPLAY_WIDTH);
        assert_eq!(img.height(), DISPLAY_HEIGHT);
    }

    #[test]
    fn test_create_blank_image_is_black() {
        let img = create_blank_image();
        let pixel = img.get_pixel(0, 0);
        assert_eq!(pixel[0], 0);
        assert_eq!(pixel[1], 0);
        assert_eq!(pixel[2], 0);
    }

    #[test]
    fn test_create_text_image_has_content() {
        let blank = create_blank_image();
        let text_img = create_text_image("Test", 14.0);

        // Text image should differ from blank (has white text)
        let blank_bytes = image_to_rgb565_bytes(&blank);
        let text_bytes = image_to_rgb565_bytes(&text_img);
        assert_ne!(blank_bytes, text_bytes);
    }

    #[test]
    fn test_rgb565_black_converts_to_zero() {
        let rgb565 = rgb_to_rgb565(0, 0, 0);
        assert_eq!(rgb565, 0x0000);
    }

    #[test]
    fn test_rgb565_white_converts_correctly() {
        let rgb565 = rgb_to_rgb565(255, 255, 255);
        // R=31, G=63, B=31 → 0xFFFF
        assert_eq!(rgb565, 0xFFFF);
    }

    #[test]
    fn test_rgb565_red_converts_correctly() {
        let rgb565 = rgb_to_rgb565(255, 0, 0);
        // R=31, G=0, B=0 → 0xF800
        assert_eq!(rgb565, 0xF800);
    }

    #[test]
    fn test_rgb565_green_converts_correctly() {
        let rgb565 = rgb_to_rgb565(0, 255, 0);
        // R=0, G=63, B=0 → 0x07E0
        assert_eq!(rgb565, 0x07E0);
    }

    #[test]
    fn test_rgb565_blue_converts_correctly() {
        let rgb565 = rgb_to_rgb565(0, 0, 255);
        // R=0, G=0, B=31 → 0x001F
        assert_eq!(rgb565, 0x001F);
    }

    #[test]
    fn test_rgb565_output_size() {
        let img = create_blank_image();
        let data = image_to_rgb565_bytes(&img);
        // 160 × 80 × 2 bytes = 25600 bytes
        let expected_size = (DISPLAY_WIDTH * DISPLAY_HEIGHT * 2) as usize;
        assert_eq!(data.len(), expected_size);
        assert_eq!(data.len(), 25600);
    }

    #[test]
    fn test_display_dimensions() {
        assert_eq!(DISPLAY_WIDTH, 160);
        assert_eq!(DISPLAY_HEIGHT, 80);
    }
}
