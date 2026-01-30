use ab_glyph::{FontRef, PxScale};
use image::{Rgb, RgbImage};
use imageproc::drawing::draw_text_mut;

/// Physical display dimensions (hardware is 80x160 portrait)
const PHYSICAL_WIDTH: u32 = 80;
const PHYSICAL_HEIGHT: u32 = 160;

/// Display orientation
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Orientation {
    /// 160x80 - wider than tall (default)
    #[default]
    Landscape,
    /// 80x160 - taller than wide
    Portrait,
}

impl Orientation {
    /// Get logical display width for this orientation
    pub fn width(self) -> u32 {
        match self {
            Orientation::Landscape => PHYSICAL_HEIGHT, // 160
            Orientation::Portrait => PHYSICAL_WIDTH,   // 80
        }
    }

    /// Get logical display height for this orientation
    pub fn height(self) -> u32 {
        match self {
            Orientation::Landscape => PHYSICAL_WIDTH, // 80
            Orientation::Portrait => PHYSICAL_HEIGHT, // 160
        }
    }
}

// Legacy constants for backward compatibility (default to landscape: 160x80)
pub const DISPLAY_WIDTH: u32 = PHYSICAL_HEIGHT; // 160
pub const DISPLAY_HEIGHT: u32 = PHYSICAL_WIDTH; // 80

#[cfg(feature = "japanese")]
const FONT_DATA: &[u8] = include_bytes!("../assets/fonts/NotoSansJP-Regular.otf");

#[cfg(not(feature = "japanese"))]
const FONT_DATA: &[u8] = include_bytes!("../assets/fonts/DejaVuSans.ttf");

pub fn create_blank_image() -> RgbImage {
    RgbImage::from_pixel(DISPLAY_WIDTH, DISPLAY_HEIGHT, Rgb([0, 0, 0]))
}

/// Create a blank image with specified orientation
pub fn create_blank_image_oriented(orientation: Orientation) -> RgbImage {
    RgbImage::from_pixel(orientation.width(), orientation.height(), Rgb([0, 0, 0]))
}

pub fn create_text_image(text: &str, font_size: f32) -> RgbImage {
    create_text_image_oriented(text, font_size, Orientation::default())
}

/// Create text image with specified orientation
pub fn create_text_image_oriented(
    text: &str,
    font_size: f32,
    orientation: Orientation,
) -> RgbImage {
    let mut img = create_blank_image_oriented(orientation);
    draw_text_oriented(&mut img, text, font_size, orientation);
    img
}

fn draw_text_oriented(img: &mut RgbImage, text: &str, font_size: f32, orientation: Orientation) {
    use ab_glyph::{Font, ScaleFont};

    let font = FontRef::try_from_slice(FONT_DATA).expect("Failed to load embedded font");
    let scale = PxScale::from(font_size);
    let scaled_font = font.as_scaled(scale);
    let line_height = scaled_font.height();

    let lines: Vec<&str> = text.lines().collect();
    let total_height = line_height * lines.len() as f32;

    let display_width = orientation.width();
    let display_height = orientation.height();

    let start_y = ((display_height as f32 - total_height) / 2.0).max(0.0) as i32;

    for (i, line) in lines.iter().enumerate() {
        let (line_width, _) = measure_text(&font, scale, line);
        let x = ((display_width as i32 - line_width as i32) / 2).max(0);
        let y = start_y + (i as f32 * line_height) as i32;

        draw_text_mut(img, Rgb([255, 255, 255]), x, y, scale, &font, line);
    }
}

pub fn measure_text_with_font_size(text: &str, font_size: f32) -> (u32, u32) {
    let font = FontRef::try_from_slice(FONT_DATA).expect("Failed to load embedded font");
    let scale = PxScale::from(font_size);
    measure_text(&font, scale, text)
}

/// Measure multi-line text dimensions at given font size.
/// Returns (max_line_width, total_height) for the text.
pub fn measure_multiline_text(text: &str, font_size: f32) -> (u32, u32) {
    use ab_glyph::{Font, ScaleFont};

    let font = FontRef::try_from_slice(FONT_DATA).expect("Failed to load embedded font");
    let scale = PxScale::from(font_size);
    let scaled_font = font.as_scaled(scale);
    let line_height = scaled_font.height();

    let lines: Vec<&str> = text.lines().collect();
    if lines.is_empty() {
        return (0, 0);
    }

    let max_width = lines
        .iter()
        .map(|line| measure_text(&font, scale, line).0)
        .max()
        .unwrap_or(0);

    let total_height = (line_height * lines.len() as f32) as u32;

    (max_width, total_height)
}

const MIN_FONT_SIZE: f32 = 8.0;
const MAX_FONT_SIZE: f32 = 72.0;
const HORIZONTAL_PADDING: u32 = 8;
const VERTICAL_PADDING: u32 = 4;

/// Calculate the largest font size that fits text within display bounds.
/// Uses binary search between MIN_FONT_SIZE (8.0) and MAX_FONT_SIZE (72.0).
pub fn calculate_auto_fit_size(text: &str) -> f32 {
    calculate_auto_fit_size_oriented(text, Orientation::default())
}

/// Calculate auto-fit size for a specific orientation
pub fn calculate_auto_fit_size_oriented(text: &str, orientation: Orientation) -> f32 {
    if text.is_empty() {
        return MIN_FONT_SIZE;
    }

    let max_text_width = orientation.width() - HORIZONTAL_PADDING;
    let max_text_height = orientation.height() - VERTICAL_PADDING;

    let mut low = MIN_FONT_SIZE;
    let mut high = MAX_FONT_SIZE;

    while high - low > 0.5 {
        let mid = (low + high) / 2.0;
        let (width, height) = measure_multiline_text(text, mid);

        if width <= max_text_width && height <= max_text_height {
            low = mid;
        } else {
            high = mid;
        }
    }

    low
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

pub fn calculate_max_chars_per_line(font_size: f32) -> usize {
    calculate_max_chars_per_line_oriented(font_size, Orientation::default())
}

pub fn calculate_max_chars_per_line_oriented(font_size: f32, orientation: Orientation) -> usize {
    let font = FontRef::try_from_slice(FONT_DATA).expect("Failed to load embedded font");
    let scale = PxScale::from(font_size);

    use ab_glyph::{Font, ScaleFont};
    let scaled_font = font.as_scaled(scale);

    // Use average character width based on 'x' (common reference character)
    let avg_width = scaled_font.h_advance(font.glyph_id('x'));

    if avg_width > 0.0 {
        (orientation.width() as f32 / avg_width).floor() as usize
    } else {
        0
    }
}

pub fn calculate_max_lines(font_size: f32) -> usize {
    calculate_max_lines_oriented(font_size, Orientation::default())
}

pub fn calculate_max_lines_oriented(font_size: f32, orientation: Orientation) -> usize {
    let font = FontRef::try_from_slice(FONT_DATA).expect("Failed to load embedded font");
    let scale = PxScale::from(font_size);

    use ab_glyph::{Font, ScaleFont};
    let scaled_font = font.as_scaled(scale);

    let line_height = scaled_font.height();

    if line_height > 0.0 {
        (orientation.height() as f32 / line_height).floor() as usize
    } else {
        0
    }
}

/// Convert image to RGB565 bytes for display (uses image dimensions)
pub fn image_to_rgb565_bytes(img: &RgbImage) -> Vec<u8> {
    image_to_rgb565_bytes_oriented(img, Orientation::default())
}

/// Convert image to RGB565 bytes, rotating if needed for landscape orientation.
/// The physical display is 80x160 (portrait), so landscape images must be rotated 90° CW.
pub fn image_to_rgb565_bytes_oriented(img: &RgbImage, orientation: Orientation) -> Vec<u8> {
    let mut data = Vec::with_capacity((PHYSICAL_WIDTH * PHYSICAL_HEIGHT * 2) as usize);

    match orientation {
        Orientation::Portrait => {
            // Portrait: send as-is (80x160)
            for y in 0..img.height() {
                for x in 0..img.width() {
                    let pixel = img.get_pixel(x, y);
                    push_rgb565(&mut data, pixel[0], pixel[1], pixel[2]);
                }
            }
        }
        Orientation::Landscape => {
            // Landscape: rotate 90° CW to fit physical 80x160 display
            // Input is 160w x 80h, output is 80w x 160h
            // Physical output scans row-by-row (py=0..160, px=0..80)
            // Maps to logical: lx = py, ly = 79 - px
            for py in 0..PHYSICAL_HEIGHT {
                for px in 0..PHYSICAL_WIDTH {
                    let lx = py;
                    let ly = (PHYSICAL_WIDTH - 1) - px; // 79 - px
                    let pixel = img.get_pixel(lx, ly);
                    push_rgb565(&mut data, pixel[0], pixel[1], pixel[2]);
                }
            }
        }
    }

    data
}

fn push_rgb565(data: &mut Vec<u8>, r: u8, g: u8, b: u8) {
    let r5 = (r >> 3) & 0x1F;
    let g6 = (g >> 2) & 0x3F;
    let b5 = (b >> 3) & 0x1F;
    let rgb565 = ((r5 as u16) << 11) | ((g6 as u16) << 5) | (b5 as u16);
    data.push((rgb565 & 0xFF) as u8);
    data.push((rgb565 >> 8) as u8);
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
        // Landscape orientation (default): 160 wide × 80 tall
        assert_eq!(DISPLAY_WIDTH, 160);
        assert_eq!(DISPLAY_HEIGHT, 80);
    }

    #[test]
    fn test_orientation_dimensions() {
        // Landscape: 160x80
        assert_eq!(Orientation::Landscape.width(), 160);
        assert_eq!(Orientation::Landscape.height(), 80);

        // Portrait: 80x160
        assert_eq!(Orientation::Portrait.width(), 80);
        assert_eq!(Orientation::Portrait.height(), 160);
    }

    #[test]
    fn test_default_orientation_is_landscape() {
        assert_eq!(Orientation::default(), Orientation::Landscape);
    }

    #[test]
    fn test_oriented_blank_image_dimensions() {
        let landscape = create_blank_image_oriented(Orientation::Landscape);
        assert_eq!(landscape.width(), 160);
        assert_eq!(landscape.height(), 80);

        let portrait = create_blank_image_oriented(Orientation::Portrait);
        assert_eq!(portrait.width(), 80);
        assert_eq!(portrait.height(), 160);
    }

    #[cfg(feature = "japanese")]
    #[test]
    fn test_japanese_text_renders() {
        let blank = create_blank_image();
        let text_img = create_text_image("こんにちは", 14.0);

        let blank_bytes = image_to_rgb565_bytes(&blank);
        let text_bytes = image_to_rgb565_bytes(&text_img);
        assert_ne!(
            blank_bytes, text_bytes,
            "Japanese text should render visible content"
        );
    }

    #[test]
    fn test_auto_fit_single_char_large() {
        let size = calculate_auto_fit_size("X");
        assert!(
            size > 40.0,
            "Single char should fit at large size, got {}",
            size
        );
    }

    #[test]
    fn test_auto_fit_long_text_smaller() {
        let size = calculate_auto_fit_size("Hello World!");
        assert!(
            size < 30.0,
            "Long text should have smaller size, got {}",
            size
        );
    }

    #[test]
    fn test_auto_fit_empty_string_min() {
        let size = calculate_auto_fit_size("");
        assert_eq!(
            size, MIN_FONT_SIZE,
            "Empty string should return MIN_FONT_SIZE"
        );
    }

    #[test]
    fn test_auto_fit_multiline_smaller() {
        let single_size = calculate_auto_fit_size("Hello");
        let multi_size = calculate_auto_fit_size("Hello\nWorld");
        assert!(
            multi_size < single_size,
            "Multi-line should be smaller than single line"
        );
    }

    #[test]
    fn test_measure_dimensions() {
        let orientation = Orientation::default();
        let max_text_width = orientation.width() - HORIZONTAL_PADDING;
        let max_text_height = orientation.height() - VERTICAL_PADDING;

        for font_size in [20.0, 40.0, 60.0, 70.0] {
            let (w, h) = measure_multiline_text("Hello", font_size);
            eprintln!(
                "Hello at {}px: {}w x {}h (max: {}x{})",
                font_size, w, h, max_text_width, max_text_height
            );
        }
        let size = calculate_auto_fit_size("Hello");
        let (w, h) = measure_multiline_text("Hello", size);
        eprintln!("Auto-fit 'Hello': size={}, dims={}x{}", size, w, h);

        assert!(
            w <= max_text_width,
            "Width {} exceeds max {}",
            w,
            max_text_width
        );
        assert!(
            h <= max_text_height,
            "Height {} exceeds max {}",
            h,
            max_text_height
        );
    }
}
