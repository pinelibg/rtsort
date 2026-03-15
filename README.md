# rtsort

`rtsort` is a work-in-progress CLI that sorts lines from standard input while rendering the growing sorted result live in the terminal.

It is intended for interactive pipelines: input is read line by line, the current sorted state is shown in an alternate screen, and the final sorted output is written to standard output when input ends.

## Status

The project is still evolving, so this README stays intentionally high level. For the exact current behavior and supported flags, use the source as the source of truth.

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

## Project layout

- `src/lib.rs` contains the core comparison and parsing logic.
- `src/main.rs` contains CLI argument parsing, terminal rendering, and the main input loop.
- `tests/integration_test.rs` exercises the compiled binary end to end.
