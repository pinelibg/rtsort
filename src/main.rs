use clap::Parser;
use crossterm::{
    cursor::{MoveToColumn, MoveUp},
    execute,
    terminal::{Clear, ClearType},
};
use rtsort::{compare_human_numeric, compare_normal};
use std::io::{self, BufRead, Write, stderr};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "A real-time sorting CLI utility",
    disable_help_flag = true
)]
struct Args {
    /// Compare according to human-readable numeric values (e.g., 2K, 1G)
    #[arg(short = 'h', long = "human-numeric-sort")]
    human_numeric_sort: bool,

    /// Reverse the result of comparisons
    #[arg(short = 'r', long = "reverse")]
    reverse: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut sorted_lines: Vec<String> = Vec::new();

    let stdin = io::stdin();
    let mut handle = stdin.lock();

    // To allow for responsive standard error/terminal manipulation even if stdout is piped
    let mut stderr = stderr();

    let mut line_buffer = String::new();
    let mut lines_to_clear: u16 = 0;

    while handle.read_line(&mut line_buffer)? > 0 {
        // Strip the trailing newline
        let original_line = line_buffer
            .trim_end_matches('\n')
            .trim_end_matches('\r')
            .to_string();

        let search_result = if args.human_numeric_sort {
            if args.reverse {
                sorted_lines.binary_search_by(|e| compare_human_numeric(e, &original_line).reverse())
            } else {
                sorted_lines.binary_search_by(|e| compare_human_numeric(e, &original_line))
            }
        } else {
            if args.reverse {
                sorted_lines.binary_search_by(|e| compare_normal(e, &original_line).reverse())
            } else {
                sorted_lines.binary_search_by(|e| compare_normal(e, &original_line))
            }
        };

        match search_result {
            Ok(pos) | Err(pos) => sorted_lines.insert(pos, original_line),
        }

        // Clear previously printed lines instead of the whole terminal
        if lines_to_clear > 0 {
            let _ = execute!(
                stderr,
                MoveUp(lines_to_clear),
                MoveToColumn(0),
                Clear(ClearType::FromCursorDown)
            );
        }

        let mut stdout = io::stdout().lock();
        for line in &sorted_lines {
            writeln!(stdout, "{line}")?;
        }
        stdout.flush()?;

        // Update the lines to clear for the next iteration
        lines_to_clear = u16::try_from(sorted_lines.len()).unwrap_or(u16::MAX);

        line_buffer.clear();
    }

    Ok(())
}
