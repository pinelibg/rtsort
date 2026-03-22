use clap::{Args, Parser};
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use rtsort::{compare_human_numeric, compare_ignore_case, compare_normal, compare_numeric};
use std::cmp::Ordering;
use std::io::{self, BufRead, Write, stderr};

#[derive(Args, Debug)]
struct SortModeArgs {
    /// Compare according to string numerical value
    #[arg(short = 'n', long = "numeric-sort")]
    numeric_sort: bool,

    /// Compare according to human-readable numeric values (e.g., 2K, 1G)
    #[arg(short = 'h', long = "human-numeric-sort")]
    human_numeric_sort: bool,

    /// Fold lower case to upper case characters for comparison
    #[arg(short = 'f', long = "ignore-case")]
    ignore_case: bool,
}

enum SortMode {
    Normal,
    Numeric,
    HumanNumeric,
    IgnoreCase,
}

impl From<&SortModeArgs> for SortMode {
    fn from(args: &SortModeArgs) -> Self {
        if args.human_numeric_sort {
            Self::HumanNumeric
        } else if args.numeric_sort {
            Self::Numeric
        } else if args.ignore_case {
            Self::IgnoreCase
        } else {
            Self::Normal
        }
    }
}

impl SortMode {
    fn comparator(&self) -> fn(&str, &str) -> Ordering {
        match self {
            Self::HumanNumeric => compare_human_numeric,
            Self::Numeric => compare_numeric,
            Self::IgnoreCase => compare_ignore_case,
            Self::Normal => compare_normal,
        }
    }
}

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "A real-time sorting CLI utility",
    disable_help_flag = true
)]
struct Cli {
    #[command(flatten)]
    sort_mode: SortModeArgs,

    /// Reverse the result of comparisons
    #[arg(short = 'r', long = "reverse")]
    reverse: bool,

    /// Output only the first N lines of the sorted result
    #[arg(short = 't', long = "top")]
    top: Option<usize>,

    /// Output only the last N lines of the sorted result
    #[arg(long = "bottom", conflicts_with = "top")]
    bottom: Option<usize>,

    /// Print help
    #[arg(long, action = clap::ArgAction::Help)]
    help: Option<bool>,
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

fn run_sort_loop(
    cmp_fn: fn(&str, &str) -> Ordering,
    reverse: bool,
    top: Option<usize>,
    bottom: Option<usize>,
) -> io::Result<Vec<String>> {
    let mut sorted_lines: Vec<String> = Vec::new();

    let stdin = io::stdin();
    let mut handle = stdin.lock();

    // To allow for responsive terminal manipulation even if stdout is piped
    let mut stderr = stderr();
    let mut guard: Option<AlternateScreenGuard> = None;

    let mut line_buffer = String::new();

    while handle.read_line(&mut line_buffer)? > 0 {
        let original_line = line_buffer.trim_end_matches(['\n', '\r']).to_string();

        if guard.is_none() {
            guard = Some(AlternateScreenGuard::new()?);
        }

        let pos = match sorted_lines.binary_search_by(|e| {
            let ord = cmp_fn(e, &original_line);
            if reverse { ord.reverse() } else { ord }
        }) {
            Ok(pos) | Err(pos) => pos,
        };

        if top.is_none_or(|n| sorted_lines.len() < n || pos < n)
            && bottom.is_none_or(|n| sorted_lines.len() < n || pos > sorted_lines.len() - n)
        {
            sorted_lines.insert(pos, original_line);
            if let Some(n) = top {
                sorted_lines.truncate(n);
            }
            if let Some(n) = bottom
                && sorted_lines.len() > n
            {
                sorted_lines.remove(0);
            }

            // Redraw from top: upstream stderr output is wiped on the next redraw
            execute!(stderr, Clear(ClearType::All), MoveTo(0, 0))?;
            for line in &sorted_lines {
                writeln!(stderr, "{line}")?;
            }
            stderr.flush()?;
        }

        line_buffer.clear();
    }

    Ok(sorted_lines)
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let cmp_fn = SortMode::from(&args.sort_mode).comparator();
    let sorted_lines = run_sort_loop(cmp_fn, args.reverse, args.top, args.bottom)?;

    let mut stdout = io::stdout().lock();
    for line in &sorted_lines {
        writeln!(stdout, "{line}")?;
    }
    stdout.flush()?;

    Ok(())
}
