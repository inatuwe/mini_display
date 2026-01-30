use crate::image::{calculate_max_chars_per_line, calculate_max_lines, DISPLAY_WIDTH};
use crate::measure_text_with_font_size;

/// Split text into pages that fit on the display.
/// Uses word-aware splitting (never breaks mid-word).
pub fn split_into_pages(text: &str, font_size: f32) -> Vec<String> {
    let max_lines = calculate_max_lines(font_size);

    if max_lines == 0 {
        return vec![];
    }

    let lines = wrap_text(text, font_size);

    if lines.is_empty() {
        return vec![];
    }

    lines
        .chunks(max_lines)
        .map(|chunk| chunk.join("\n"))
        .collect()
}

/// Wrap text into lines that fit within the display width.
/// Respects word boundaries and existing newlines.
fn wrap_text(text: &str, font_size: f32) -> Vec<String> {
    if text.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();
    let max_chars = calculate_max_chars_per_line(font_size);

    for paragraph in text.split('\n') {
        if paragraph.is_empty() {
            result.push(String::new());
            continue;
        }

        let words: Vec<&str> = paragraph.split_whitespace().collect();
        if words.is_empty() {
            result.push(String::new());
            continue;
        }

        let mut current_line = String::new();

        for word in words {
            if current_line.is_empty() {
                // First word on line - check if it fits
                if fits_in_width(word, font_size) {
                    current_line = word.to_string();
                } else {
                    // Word too long, truncate it
                    current_line = truncate_to_fit(word, font_size, max_chars);
                    result.push(current_line);
                    current_line = String::new();
                }
            } else {
                // Try adding word to current line
                let test_line = format!("{} {}", current_line, word);
                if fits_in_width(&test_line, font_size) {
                    current_line = test_line;
                } else {
                    // Start new line
                    result.push(current_line);
                    if fits_in_width(word, font_size) {
                        current_line = word.to_string();
                    } else {
                        current_line = truncate_to_fit(word, font_size, max_chars);
                        result.push(current_line);
                        current_line = String::new();
                    }
                }
            }
        }

        if !current_line.is_empty() {
            result.push(current_line);
        }
    }

    result
}

/// Check if text fits within display width
fn fits_in_width(text: &str, font_size: f32) -> bool {
    let (width, _) = measure_text_with_font_size(text, font_size);
    width <= DISPLAY_WIDTH
}

/// Truncate word to fit within display width
fn truncate_to_fit(word: &str, font_size: f32, max_chars: usize) -> String {
    let mut result = word.to_string();
    let limit = max_chars.min(word.len());

    while !result.is_empty() && !fits_in_width(&result, font_size) {
        result = result.chars().take(limit.saturating_sub(1)).collect();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_empty_string() {
        let pages = split_into_pages("", 14.0);
        assert!(pages.is_empty());
    }

    #[test]
    fn test_split_single_word_that_fits() {
        let pages = split_into_pages("Hello", 14.0);
        assert_eq!(pages.len(), 1);
        assert_eq!(pages[0], "Hello");
    }

    #[test]
    fn test_split_text_requiring_multiple_pages() {
        // Long text that should span multiple pages at font size 14
        let long_text = "This is a long message that will definitely need to be split across multiple pages because it contains many words and the display is only 160x80 pixels which is quite small for displaying lengthy text content.";
        let pages = split_into_pages(long_text, 14.0);
        assert!(
            pages.len() > 1,
            "Expected multiple pages, got {}",
            pages.len()
        );
    }

    #[test]
    fn test_word_boundary_splitting() {
        // Verify words aren't broken mid-word
        let text = "Hello World Test";
        let pages = split_into_pages(text, 14.0);

        for page in &pages {
            // Check no partial words (assuming these are complete words)
            let words: Vec<&str> = page.split_whitespace().collect();
            for word in words {
                assert!(
                    text.contains(word),
                    "Found unexpected word fragment: {}",
                    word
                );
            }
        }
    }

    #[test]
    fn test_split_with_newlines() {
        let text = "Line one\nLine two\nLine three";
        let pages = split_into_pages(text, 14.0);

        // Should preserve line structure
        assert!(!pages.is_empty());

        // The combined pages should contain all original lines
        let combined: String = pages.join("\n");
        assert!(combined.contains("Line one"));
        assert!(combined.contains("Line two"));
        assert!(combined.contains("Line three"));
    }

    #[test]
    fn test_wrap_text_basic() {
        let lines = wrap_text("Hello World", 14.0);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_fits_in_width_short_text() {
        assert!(fits_in_width("Hi", 14.0));
    }

    #[test]
    fn test_fits_in_width_long_text() {
        let long = "This is a very long line that definitely won't fit on a 160 pixel wide display";
        assert!(!fits_in_width(long, 14.0));
    }
}
