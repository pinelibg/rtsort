use clap::Parser;
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use rtsort::{compare_human_numeric, compare_normal};
use std::cmp::Ordering;
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

struct AlternateScreenGuard;

impl AlternateScreenGuard {
    fn new() -> io::Result<Self> {
        execute!(stderr(), EnterAlternateScreen)?;
        Ok(Self)
    }
}

impl Drop for AlternateScreenGuard {
    fn drop(&mut self) {
        let _ = execute!(stderr(), LeaveAlternateScreen);
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut sorted_lines: Vec<String> = Vec::new();

    let stdin = io::stdin();
    let mut handle = stdin.lock();

    // To allow for responsive terminal manipulation even if stdout is piped
    let mut stderr = stderr();
    let guard = AlternateScreenGuard::new()?;

    let cmp_fn: fn(&str, &str) -> Ordering = if args.human_numeric_sort {
        compare_human_numeric
    } else {
        compare_normal
    };

    let mut line_buffer = String::new();

    while handle.read_line(&mut line_buffer)? > 0 {
        // Strip the trailing newline
        let original_line = line_buffer.trim_end_matches(['\n', '\r']).to_string();

        let search_result = sorted_lines.binary_search_by(|e| {
            let ord = cmp_fn(e, &original_line);
            if args.reverse { ord.reverse() } else { ord }
        });

        match search_result {
            Ok(pos) | Err(pos) => sorted_lines.insert(pos, original_line),
        }

        // Redraw from top: upstream stderr output is wiped on the next redraw
        execute!(stderr, Clear(ClearType::All), MoveTo(0, 0))?;
        for line in &sorted_lines {
            writeln!(stderr, "{line}")?;
        }
        stderr.flush()?;

        line_buffer.clear();
    }

    // Leave alternate screen, then print the final sorted list to stdout
    drop(guard);

    let mut stdout = io::stdout().lock();
    for line in &sorted_lines {
        writeln!(stdout, "{line}")?;
    }
    stdout.flush()?;

    Ok(())
}
