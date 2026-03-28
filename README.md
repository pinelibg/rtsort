# rtsort

A real-time `sort` with a live terminal preview.

![demo](assets/demo.gif)

`rtsort` is a streaming alternative to the standard `sort` command. It displays a continuously updated preview of your sorted data as it arrives, then seamlessly writes the final output to stdout.

## Features

- Live terminal display updates as each line arrives
- Alphabetical, numeric, human-readable numeric, and version number sort modes
- Case-insensitive sorting
- Reverse ordering
- Limit output to the top or bottom N results
- Deduplicate lines that compare equal under the active sort mode (like `sort -u`)
- `--no-preview` mode for scripting without the live terminal display
- Configurable preview update rate via `--fps`
- Sort by a specific field with a configurable field delimiter

## Install

### Install with Cargo from crates.io

```bash
cargo install rtsort
```

### Download a release binary

Download the archive for your platform from the [GitHub Releases page](https://github.com/pinelibg/rtsort/releases), extract it, and place the `rtsort` binary somewhere on your `PATH`.

### Install with Cargo from the repository

```bash
cargo install --git https://github.com/pinelibg/rtsort.git
```

If your GitHub access uses SSH:

```bash
cargo install --git ssh://git@github.com/pinelibg/rtsort.git
```

### Build from a local clone

```bash
git clone https://github.com/pinelibg/rtsort.git
cd rtsort
cargo install --path .
```

## Usage

```
rtsort [OPTIONS]

Options:
  -n, --numeric-sort          Compare by string numerical value
  -h, --human-numeric-sort    Compare by human-readable numeric values (e.g. 2K, 1G)
  -f, --ignore-case           Fold lower case to upper case characters for comparison
  -V, --version-sort          Sort by version numbers (e.g. 1.9 < 1.10)
  -r, --reverse               Reverse the sort order
  -u, --unique                Remove lines that compare equal under the active sort mode
      --top <N>               Output only the first N lines of the sorted result
      --bottom <N>            Output only the last N lines of the sorted result
      --no-preview            Suppress the live terminal preview (no alternate screen)
      --fps <N>               Preview update rate in frames per second (default: 30; 0 = every line)
  -k, --key <N>               Sort by field N (1-indexed)
  -t, --field-separator <CHAR> Field delimiter character (used with -k; default: whitespace)
      --help                  Print help
```

### Examples

```bash
# Alphabetical sort (default)
printf 'pear\napple\nbanana\n' | rtsort

# Numeric sort
printf '10\n2\n30\n' | rtsort -n

# Human-readable numeric sort
printf '1K\n50\n2M\n' | rtsort -h

# Version sort (1.9 < 1.10 < 2.0)
printf 'v1.10\nv1.9\nv2.0\nv1.0\n' | rtsort -V

# Case-insensitive sort
printf 'banana\nApple\ncherry\n' | rtsort -f

# Reverse sort
printf 'a\nc\nb\n' | rtsort -r

# Show only the top 3 results
printf '5\n1\n4\n2\n3\n' | rtsort -n --top 3

# Show only the bottom 3 results
printf '5\n1\n4\n2\n3\n' | rtsort -n --bottom 3

# Remove duplicate lines
printf 'b\na\nb\na\n' | rtsort -u

# Sort by the second whitespace-delimited field
printf 'foo 3\nbar 1\nbaz 2\n' | rtsort -k 2

# Sort by the second colon-delimited field
printf 'foo:3\nbar:1\nbaz:2\n' | rtsort -k 2 -t ':'

# Sort directories by size (human-readable)
du -sh */ | rtsort -h

# Find the 5 largest directories
du -sh */ | rtsort -hr --top 5

# Sort without live terminal preview (useful in scripts)
printf 'pear\napple\nbanana\n' | rtsort --no-preview

# Limit preview to 10 fps (useful on slow terminals)
printf 'pear\napple\nbanana\n' | rtsort --fps 10

# Slow preview for very long-running streams
printf 'pear\napple\nbanana\n' | rtsort --fps 0.5

# Update preview on every line (no rate limiting)
printf 'pear\napple\nbanana\n' | rtsort --fps 0
```

## Development

```bash
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

To try without installing:

```bash
printf 'pear\napple\nbanana\n' | cargo run
printf '10\n2\n30\n' | cargo run -- -n
```
