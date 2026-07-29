use clap::{ArgGroup, Args, Parser};
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use rtsort::{SortPolicy, SortedBuffer, comparator};
use std::cmp::Ordering;
use std::io::{self, BufRead, Write, stderr};
use std::num::NonZeroUsize;
use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};
use std::time::{Duration, Instant};

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

#[derive(Clone, Copy)]
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
    fn comparator(self) -> fn(&str, &str) -> Ordering {
        match self {
            Self::HumanNumeric => comparator::compare_human_numeric,
            Self::Numeric => comparator::compare_numeric,
            Self::IgnoreCase => comparator::compare_ignore_case,
            Self::Version => comparator::compare_version,
            Self::Normal => comparator::compare_normal,
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

    /// Preview update rate in frames per second (0 = update on every line)
    #[arg(long = "fps", default_value_t = 30.0, value_parser = parse_fps)]
    fps: f64,

    /// Sort by field N (1-indexed)
    #[arg(short = 'k', long = "key", value_parser = parse_key_field)]
    key: Option<NonZeroUsize>,

    /// Field delimiter character (used with -k; default: whitespace)
    #[arg(short = 't', long = "field-separator", requires = "key")]
    field_sep: Option<char>,

    /// Remove duplicate lines from the sorted output
    #[arg(short = 'u', long = "unique")]
    unique: bool,

    /// Print help
    #[arg(long, action = clap::ArgAction::Help)]
    help: Option<bool>,

    /// Print version
    #[arg(long, action = clap::ArgAction::Version)]
    version: Option<bool>,
}

fn parse_key_field(s: &str) -> Result<NonZeroUsize, String> {
    let n: usize = s
        .parse()
        .map_err(|_| format!("`{s}` is not a valid field number"))?;
    NonZeroUsize::new(n).ok_or_else(|| "field number must be 1 or greater".to_string())
}

fn parse_fps(s: &str) -> Result<f64, String> {
    let fps: f64 = s
        .parse()
        .map_err(|_| format!("`{s}` is not a valid number"))?;
    if fps.is_finite() && fps >= 0.0 {
        Ok(fps)
    } else {
        Err("fps must be a non-negative finite number".to_string())
    }
}

static IN_ALTERNATE_SCREEN: AtomicBool = AtomicBool::new(false);

struct AlternateScreenGuard;

impl AlternateScreenGuard {
    fn new() -> io::Result<Self> {
        IN_ALTERNATE_SCREEN.store(true, AtomicOrdering::SeqCst);
        if let Err(e) = execute!(stderr(), EnterAlternateScreen) {
            IN_ALTERNATE_SCREEN.store(false, AtomicOrdering::SeqCst);
            return Err(e);
        }
        Ok(Self)
    }
}

impl Drop for AlternateScreenGuard {
    fn drop(&mut self) {
        let _ = execute!(stderr(), LeaveAlternateScreen);
        IN_ALTERNATE_SCREEN.store(false, AtomicOrdering::SeqCst);
    }
}

/// Redraws the preview on the alternate screen, capped to the terminal height.
/// Redraws from the top: upstream stderr output is wiped on the next redraw.
fn render_preview(stderr: &mut io::Stderr, buffer: &SortedBuffer) -> io::Result<()> {
    let rows = crossterm::terminal::size().map_or(usize::MAX, |(_, r)| r as usize);
    execute!(stderr, Clear(ClearType::All), MoveTo(0, 0))?;
    // The last visible line is written without a newline to avoid scrolling
    let mut preview = buffer.lines().take(rows).peekable();
    while let Some(line) = preview.next() {
        if preview.peek().is_some() {
            writeln!(stderr, "{line}")?;
        } else {
            write!(stderr, "{line}")?;
        }
    }
    stderr.flush()
}

fn run_sort_loop(args: &Cli) -> io::Result<Vec<String>> {
    let mut buffer = SortedBuffer::new(SortPolicy {
        cmp_fn: SortMode::from(&args.sort_mode).comparator(),
        reverse: args.reverse,
        unique: args.unique,
        top: args.top,
        bottom: args.bottom,
        key_field: args.key,
        field_sep: args.field_sep,
    });

    let stdin = io::stdin();
    let mut handle = stdin.lock();

    // To allow for responsive terminal manipulation even if stdout is piped
    let mut stderr = stderr();
    let mut guard: Option<AlternateScreenGuard> = None;

    let render_interval = (args.fps > 0.0).then(|| Duration::from_secs_f64(1.0 / args.fps));

    let mut line_buffer = String::new();
    let mut last_render: Option<Instant> = None;

    while handle.read_line(&mut line_buffer)? > 0 {
        let original_line = line_buffer.trim_end_matches(['\n', '\r']);

        if !args.no_preview && guard.is_none() {
            guard = Some(AlternateScreenGuard::new()?);
        }

        if buffer.insert(original_line) && !args.no_preview {
            let should_render = render_interval
                .is_none_or(|interval| last_render.is_none_or(|t| t.elapsed() >= interval));
            if should_render {
                render_preview(&mut stderr, &buffer)?;
                last_render = Some(Instant::now());
            }
        }

        line_buffer.clear();
    }

    // Final render to refresh the preview and keep it within terminal height before leaving
    if !args.no_preview && guard.is_some() {
        render_preview(&mut stderr, &buffer)?;
    }

    Ok(buffer.into_lines())
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    ctrlc::set_handler(|| {
        if IN_ALTERNATE_SCREEN.load(AtomicOrdering::SeqCst) {
            let _ = execute!(stderr(), LeaveAlternateScreen);
        }
        std::process::exit(130);
    })
    .map_err(|e| io::Error::other(format!("Error setting Ctrl-C handler: {e}")))?;

    let sorted_lines = run_sort_loop(&args)?;

    let mut stdout = io::stdout().lock();
    for line in &sorted_lines {
        writeln!(stdout, "{line}")?;
    }
    stdout.flush()?;

    Ok(())
}
