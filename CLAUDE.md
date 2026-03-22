# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**rtsort** is a real-time sorting CLI tool written in Rust. It reads stdin line-by-line, maintains a sorted list rendered live in an alternate terminal screen, then writes the final sorted output to stdout on EOF.

## Commands

```bash
cargo build
cargo test
cargo test <test_name>          # run a single test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo fmt
```

## Architecture

- **`src/lib.rs`** — Pure sorting logic, no I/O
- **`src/main.rs`** — CLI argument parsing, terminal rendering, main sort loop

Integration tests in `tests/integration_test.rs` run the compiled binary end-to-end via `assert_cmd`.

## Documentation

When adding new CLI options or features, update `README.md` to reflect the changes:

- **Features** section — add a bullet for the new capability
- **Usage > Options** — add the new flag with its description
- **Usage > Examples** — add a usage example

## Git Hooks

Pre-commit and pre-push hooks run fmt, clippy, and tests via `hk` (configured in `hk.pkl`, managed by `mise`).
