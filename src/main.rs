use clap::{ArgGroup, Args, Parser};
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use rtsort::{
    compare_human_numeric, compare_ignore_case, compare_normal, compare_numeric, compare_version,
    extract_key_field,
};
use std::cmp::Ordering;
use std::io::{self, BufRead, Write, stderr};

#[derive(Args, Debug)]
#[allow(clippy::struct_excessive_bools)]
#[command(group(ArgGroup::new("sort_mode").multiple(false)))]
struct SortModeArgs {
    /// Compare according to string numerical value
    #[arg(short = 'n', long = "numeric-sort", group = "sort_mode")]
    numeric_sort: bool,

    /// Compare according to human-readable numeric values (e.g., 2K, 1G)
    #[arg(short = 'h', long = "human-numeric-sort", group = "sort_mode")]
    human_numeric_sort: bool,

    /// Fold lower case to upper case characters for comparison
    #[arg(short = 'f', long = "ignore-case", group = "sort_mode")]
    ignore_case: bool,

    /// Sort by version numbers (e.g., 1.9 < 1.10)
    #[arg(short = 'V', long = "version-sort", group = "sort_mode")]
    version_sort: bool,
}

enum SortMode {
    Normal,
    Numeric,
    HumanNumeric,
    IgnoreCase,
    Version,
}

impl From<&SortModeArgs> for SortMode {
    fn from(args: &SortModeArgs) -> Self {
        if args.human_numeric_sort {
            Self::HumanNumeric
        } else if args.numeric_sort {
            Self::Numeric
        } else if args.ignore_case {
            Self::IgnoreCase
        } else if args.version_sort {
            Self::Version
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
            Self::Version => compare_version,
            Self::Normal => compare_normal,
        }
    }
}

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "A real-time sorting CLI utility",
    disable_help_flag = true,
    disable_version_flag = true
)]
struct Cli {
    #[command(flatten)]
    sort_mode: SortModeArgs,

    /// Reverse the result of comparisons
    #[arg(short = 'r', long = "reverse")]
    reverse: bool,

    /// Output only the first N lines of the sorted result
    #[arg(long = "top")]
    top: Option<usize>,

    /// Output only the last N lines of the sorted result
    #[arg(long = "bottom", conflicts_with = "top")]
    bottom: Option<usize>,

    /// Suppress the live terminal preview (no alternate screen)
    #[arg(long = "no-preview")]
    no_preview: bool,

    /// Sort by field N (1-indexed)
    #[arg(short = 'k', long = "key", value_parser = parse_key_field)]
    key: Option<usize>,

    /// Field delimiter character (used with -k; default: whitespace)
    #[arg(short = 't', long = "field-separator", requires = "key")]
    field_sep: Option<char>,

    /// Print help
    #[arg(long, action = clap::ArgAction::Help)]
    help: Option<bool>,

    /// Print version
    #[arg(long, action = clap::ArgAction::Version)]
    version: Option<bool>,
}

fn parse_key_field(s: &str) -> Result<usize, String> {
    let n: usize = s
        .parse()
        .map_err(|_| format!("`{s}` is not a valid field number"))?;
    if n == 0 {
        return Err("field number must be 1 or greater".to_string());
    }
    Ok(n)
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
    cmp_fn: impl Fn(&str, &str) -> Ordering,
    reverse: bool,
    top: Option<usize>,
    bottom: Option<usize>,
    no_preview: bool,
    key: Option<usize>,
    field_sep: Option<char>,
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

        if !no_preview && guard.is_none() {
            guard = Some(AlternateScreenGuard::new()?);
        }

        let pos = match sorted_lines.binary_search_by(|e| {
            let (key_e, key_line) = match key {
                Some(n) => (
                    extract_key_field(e, n, field_sep),
                    extract_key_field(&original_line, n, field_sep),
                ),
                None => (e.as_str(), original_line.as_str()),
            };
            let ord = cmp_fn(key_e, key_line);
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

            if !no_preview {
                // Redraw from top: upstream stderr output is wiped on the next redraw
                execute!(stderr, Clear(ClearType::All), MoveTo(0, 0))?;
                for line in &sorted_lines {
                    writeln!(stderr, "{line}")?;
                }
                stderr.flush()?;
            }
        }

        line_buffer.clear();
    }

    Ok(sorted_lines)
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let cmp_fn = SortMode::from(&args.sort_mode).comparator();
    let sorted_lines = run_sort_loop(
        cmp_fn,
        args.reverse,
        args.top,
        args.bottom,
        args.no_preview,
        args.key,
        args.field_sep,
    )?;

    let mut stdout = io::stdout().lock();
    for line in &sorted_lines {
        writeln!(stdout, "{line}")?;
    }
    stdout.flush()?;

    Ok(())
}
