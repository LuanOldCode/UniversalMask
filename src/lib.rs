// src/lib.rs

/// Applies a mask to text according to the specified format.
///
/// Accepts one or more mask formats (separated by '|') and applies the most suitable one
/// based on the length of the input text.
///
/// # Examples
///
/// ```
/// // SSN: XXX-XX-XXXX
/// let ssn = mask("123456789", "XXX-XX-XXXX");
/// assert_eq!(ssn, "123-45-6789");
///
/// // Phone: (XXX) XXX-XXXX
/// let phone = mask("1234567890", "(XXX) XXX-XXXX");
/// assert_eq!(phone, "(123) 456-7890");
///
/// // Multiple formats: SSN or EIN
/// let number = mask("123456789012", "XXX-XX-XXXX | XX-XXXXXXX");
/// assert_eq!(number, "12-3456789");
///
/// // Multiple formats with various patterns
/// let multi_format = mask("123456789012345", "XXX-XX-XXXX | XX-XXXXXXX | XXX-XXX-XXX-XXX-XXX");
/// assert_eq!(multi_format, "123-456-789-012-345");
/// ```
pub fn mask(text: &str, format_patterns: &str) -> String {
    // Return empty string if the input text is empty
    if text.is_empty() {
        return String::new();
    }

    // Pre-allocate bytes instead of using Vec
    let text_bytes = text.as_bytes();
    let text_len = text_bytes.len();

    // Split format patterns and find the best format
    let mut best_format = "";
    let mut best_x_count = 0;

    for format in format_patterns.split('|') {
        let format = format.trim();
        let x_count = format.bytes().filter(|&b| b == b'X').count();

        if best_format.is_empty() || (text_len > best_x_count && x_count > best_x_count) {
            best_format = format;
            best_x_count = x_count;
        }
    }

    // Log a warning if the text has more characters than the format can handle
    if text_len > best_x_count {
        eprintln!("Warning: The provided text has more characters ({}) than the format can handle ({})",
                 text_len, best_x_count);
    }

    // Pre-allocate result string with estimated capacity
    let mut result = String::with_capacity(best_format.len());
    let mut text_index = 0;

    // Use bytes for faster iteration where possible
    for &b in best_format.as_bytes() {
        if b == b'X' {
            if text_index < text_len {
                result.push(text_bytes[text_index] as char);
                text_index += 1;
            } else {
                break;
            }
        } else {
            result.push(b as char);
        }
    }

    result
}
