use similar::{Algorithm, DiffTag, TextDiff, ChangeTag};
use crate::theme::Theme;

/// A single line (or line pair) of diff output.
pub struct DiffLine {
    pub left_num: Option<usize>,
    pub left_text: Option<String>,
    pub right_num: Option<usize>,
    pub right_text: Option<String>,
    /// now unused for coloring, but kept for future metadata
    pub left_diff: bool,
    pub right_diff: bool,
}

/// Compute a side-by-side diff of two text files, returning a list of `DiffLine`s.
pub fn compute_diff(text1: &str, text2: &str) -> Vec<DiffLine> {
    if text1 == text2 {
        return vec![];
    }
    
    let old_lines: Vec<&str> = text1.lines().collect();
    let new_lines: Vec<&str> = text2.lines().collect();
    let ops = similar::capture_diff_slices(Algorithm::Myers, &old_lines, &new_lines);

    // Only Dracula now
    let theme = Theme::dracula();
    let mut diff_lines = Vec::new();

    for op in ops {
        let (tag, old_range, new_range) = op.as_tag_tuple();
        match tag {
            DiffTag::Equal => {
                for offset in 0..(old_range.end - old_range.start) {
                    let i = old_range.start + offset;
                    let j = new_range.start + offset;
                    diff_lines.push(DiffLine {
                        left_num: Some(i + 1),
                        left_text: Some(old_lines[i].to_string()),
                        right_num: Some(j + 1),
                        right_text: Some(new_lines[j].to_string()),
                        left_diff: false,
                        right_diff: false,
                    });
                }
            }

            DiffTag::Delete => {
                // Word-diff original vs empty
                for i in old_range.start..old_range.end {
                    let original = old_lines[i];
                    let styled = apply_word_style(original, "", &theme);
                    diff_lines.push(DiffLine {
                        left_num: Some(i + 1),
                        left_text: Some(styled.left),
                        right_num: None,
                        right_text: None,
                        left_diff: false,
                        right_diff: false,
                    });
                }
            }

            DiffTag::Insert => {
                // Word-diff empty vs new
                for j in new_range.start..new_range.end {
                    let added = new_lines[j];
                    let styled = apply_word_style("", added, &theme);
                    diff_lines.push(DiffLine {
                        left_num: None,
                        left_text: None,
                        right_num: Some(j + 1),
                        right_text: Some(styled.right),
                        left_diff: false,
                        right_diff: false,
                    });
                }
            }

            DiffTag::Replace => {
                // Line-level replace: word-diff each matching pair (or lone line)
                let old_len = old_range.end - old_range.start;
                let new_len = new_range.end - new_range.start;
                let line_count = old_len.max(new_len);

                for k in 0..line_count {
                    let i_opt = if k < old_len { Some(old_range.start + k) } else { None };
                    let j_opt = if k < new_len { Some(new_range.start + k) } else { None };

                    let left_line = i_opt.map(|i| old_lines[i]).unwrap_or("");
                    let right_line = j_opt.map(|j| new_lines[j]).unwrap_or("");
                    let styled = apply_word_style(left_line, right_line, &theme);

                    diff_lines.push(DiffLine {
                        left_num: i_opt.map(|i| i + 1),
                        left_text: Some(styled.left),
                        right_num: j_opt.map(|j| j + 1),
                        right_text: Some(styled.right),
                        left_diff: false,
                        right_diff: false,
                    });
                }
            }
        }
    }

    diff_lines
}

/// Intermediate result of styling two lines word-by-word.
struct StyledPair {
    left: String,
    right: String,
}

/// Do a word-level diff on `left` vs `right`, and return
/// two strings where only the changed words are styled.
fn apply_word_style(left: &str, right: &str, theme: &Theme) -> StyledPair {
    let word_diff = TextDiff::from_words(left, right);
    let mut styled_left = String::new();
    let mut styled_right = String::new();

    for op in word_diff.ops() {
        for change in word_diff.iter_changes(op) {
            match change.tag() {
                ChangeTag::Equal => {
                    styled_left.push_str(change.value());
                    styled_right.push_str(change.value());
                }
                ChangeTag::Delete => {
                    styled_left.push_str(&theme.deletion.apply_to(change.value()).to_string());
                }
                ChangeTag::Insert => {
                    styled_right.push_str(&theme.addition.apply_to(change.value()).to_string());
                }
            }
        }
    }

    StyledPair { left: styled_left, right: styled_right }
}
