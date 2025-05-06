mod diff;
mod utils;
mod theme;

use clap::Parser;
use std::fs;
use pager::Pager;
use textwrap::termwidth;

/// Simple side‑by‑side diff viewer (Dracula only).
#[derive(Parser, Debug)]
#[command(name = "diffside", about = "Side-by-side diff tool", version)]
struct Args {
    /// First file to compare
    file1: String,

    /// Second file to compare
    file2: String,

    /// Disable paging through less
    #[arg(long, default_value_t = false)]
    no_pager: bool,
}

fn main() {
    console::set_colors_enabled(true);
    let args = Args::parse();
    // let theme = theme::Theme::dracula();

    // Read both files (or exit with error)
    let text1 = fs::read_to_string(&args.file1).unwrap_or_else(|e| {
        eprintln!("Error reading {}: {}", &args.file1, e);
        std::process::exit(1);
    });
    let text2 = fs::read_to_string(&args.file2).unwrap_or_else(|e| {
        eprintln!("Error reading {}: {}", &args.file2, e);
        std::process::exit(1);
    });

    // Set up pager if desired
    if !args.no_pager {
        Pager::with_pager("less -R").setup();
    }

    // Compute all DiffLine entries (with inline ANSI styling)
    let diff_lines = diff::compute_diff(&text1, &text2);
    if diff_lines.is_empty() {
        // println!("✅ No differences found.");
        println!("✅ No differences found between '{}' and '{}'.", args.file1, args.file2);
        return;
    }

    // Determine width for line number columns
    let left_num_width = diff_lines
        .iter()
        .filter_map(|dl| dl.left_num)
        .max()
        .unwrap_or(0)
        .to_string()
        .len();
    let right_num_width = diff_lines
        .iter()
        .filter_map(|dl| dl.right_num)
        .max()
        .unwrap_or(0)
        .to_string()
        .len();

    // Compute how much width remains for each side's text
    let tw = termwidth();
    let reserve = left_num_width + right_num_width + 3; // spaces and separator
    let mut content_width = if tw > reserve { (tw - reserve) / 2 } else { 80 };
    if content_width < 10 {
        content_width = 10;
    }

    // Print each diff line, wrapping the pre‑styled text
    for dl in diff_lines {
        // Convert Option<usize> to strings (or blank)
        let ln = dl.left_num.map_or(String::new(), |n| n.to_string());
        let rn = dl.right_num.map_or(String::new(), |n| n.to_string());

        // Wrap each side's text into equal-width segments
        let left_segs = utils::wrap_text(dl.left_text.as_deref().unwrap_or(""), content_width);
        let right_segs = utils::wrap_text(dl.right_text.as_deref().unwrap_or(""), content_width);

        // Print as many rows as the taller side
        let rows = left_segs.len().max(right_segs.len());
        for i in 0..rows {
            // grab or fill with spaces
            let left = left_segs
                .get(i)
                .map(|s| {
                    let s = s.strip_suffix('\n').unwrap_or(s);
                    s.to_string()
                })
                .unwrap_or_else(|| " ".repeat(content_width));

            let right = right_segs
                .get(i)
                .map(|s| {
                    let s = s.strip_suffix('\n').unwrap_or(s);
                    s.to_string()
                })
                .unwrap_or_else(|| " ".repeat(content_width));

            // only show line numbers on the first wrapped row
            let (l_num, r_num) = if i == 0 {
                (ln.as_str(), rn.as_str())
            } else {
                ("", "")
            };

            // Print: right‑aligned line numbers, a space, text, " | ", then the other side
            println!(
                "{:>lwidth$} {} | {:>rwidth$} {}",
                l_num,
                left,
                r_num,
                right,
                lwidth = left_num_width,
                rwidth = right_num_width,
            );
        }
    }
}
