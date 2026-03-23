# rtsort

A real-time `sort` with a live terminal preview.

![demo](assets/demo.gif)

`rtsort` is a streaming alternative to the standard `sort` command. It displays a continuously updated preview of your sorted data as it arrives, then seamlessly writes the final output to stdout.

## Features

- Live terminal display updates as each line arrives
- Alphabetical, numeric, and human-readable numeric sort modes
- Case-insensitive sorting
- Reverse ordering
- Limit output to the top or bottom N results
- `--no-preview` mode for scripting without the live terminal display

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
  -r, --reverse               Reverse the sort order
  -t, --top <N>               Output only the first N lines of the sorted result
      --bottom <N>            Output only the last N lines of the sorted result
      --no-preview            Suppress the live terminal preview (no alternate screen)
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

# Case-insensitive sort
printf 'banana\nApple\ncherry\n' | rtsort -f

# Reverse sort
printf 'a\nc\nb\n' | rtsort -r

# Show only the top 3 results
printf '5\n1\n4\n2\n3\n' | rtsort -n --top 3

# Show only the bottom 3 results
printf '5\n1\n4\n2\n3\n' | rtsort -n --bottom 3

# Sort directories by size (human-readable)
du -sh */ | rtsort -h

# Find the 5 largest directories
du -sh */ | rtsort -hr --top 5

# Sort without live terminal preview (useful in scripts)
printf 'pear\napple\nbanana\n' | rtsort --no-preview
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
