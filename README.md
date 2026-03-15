# rtsort

`rtsort` is a work-in-progress CLI that sorts lines from standard input while rendering the growing sorted result live in the terminal.

It is intended for interactive pipelines: input is read line by line, the current sorted state is shown in an alternate screen, and the final sorted output is written to standard output when input ends.

## Status

The project is still evolving, so this README stays intentionally high level. For the exact current behavior and supported flags, use the source as the source of truth.

## Install

Choose the option that fits how you want to use the tool.

### Download a release binary

If a release is available for your platform, download the archive from the [GitHub Releases page](https://github.com/pinelibg/rtsort/releases), extract it, and place the `rtsort` binary somewhere on your `PATH`.

### Install from the private repository with Cargo

This is the simplest option if you already use Rust and have access to the private repository:

```bash
cargo install --git https://github.com/pinelibg/rtsort.git
```

If your GitHub access is configured over SSH, you can use:

```bash
cargo install --git ssh://git@github.com/pinelibg/rtsort.git
```

### Build from a local clone

If you want to work from a checked out copy of the repository:

```bash
git clone https://github.com/pinelibg/rtsort.git
cd rtsort
cargo install --path .
```

### Run without installing

To try it directly from the repository:

```bash
cargo run
```

## Quick start

Run from the repository with Cargo:

```bash
printf 'pear\napple\nbanana\n' | cargo run
```

You can also pass the current sort mode flags directly to the binary. For example:

```bash
printf '10\n2\n30\n' | cargo run -- --numeric-sort
printf '1K\n50\n2M\n' | cargo run -- --human-numeric-sort
printf 'a\nc\nb\n' | cargo run -- --reverse
```

## Development

Common commands:

```bash
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```
