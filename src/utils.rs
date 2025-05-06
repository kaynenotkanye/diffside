use console::measure_text_width;
use textwrap;
use textwrap::wrap;


/// Pad a string with spaces on the right to reach the specified width.
pub fn pad_str(s: &str, width: usize) -> String {
    let actual_width = measure_text_width(s);
    if actual_width >= width {
        // Truncate if somehow longer (should not happen if wrapping is correct)
        s.chars().take(width).collect()
    } else {
        let mut result = String::from(s);
        result.push_str(&" ".repeat(width - actual_width));
        result
    }
}

/*
/// Wraps text to the specified width and returns a vector of lines,
/// with any trailing newlines stripped to prevent extra line feeds.
pub fn wrap_text(text: &str, width: usize) -> Vec<String> {
    wrap(text, width)
        .into_iter()
        .map(|s| {
            // Ensure each wrapped line has no trailing newlines, but retains spaces
            s.trim_end_matches('\n').to_string()
        })
        .collect()
}
*/

/// Wrap a line of text to the given column width, returning a vector of lines.
/// Each returned line is padded to exactly `width` columns.
pub fn wrap_text(text: &str, width: usize) -> Vec<String> {
    if text.is_empty() {
        // If the text is empty, return a single line of spaces (to preserve a blank line)
        return vec![" ".repeat(width)];
    }
    // Use textwrap to wrap the text into lines (breaks at word boundaries and long words)
    let wrapped: Vec<String> = textwrap::wrap(text, width)
        .into_iter()
        .map(|cow| cow.into_owned())
        .collect();
    // Ensure at least one line (if text consists of only newline, wrap may give empty vec)
    if wrapped.is_empty() {
        return vec![" ".repeat(width)];
    }
    // Pad each wrapped line to full width for alignment
    wrapped.into_iter().map(|line| pad_str(&line, width)).collect()
}
