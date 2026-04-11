# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

PULSAR (**Platform for Unified Learning through Systems Architecture in Rust**) is an interactive REPL and shell environment for teaching mathematics, Rust, and CS concepts through hands-on learning. The project is in early development — most crates contain placeholder scaffolding.

## Commands

```bash
# Build all crates
cargo build

# Build release binaries
cargo build --release

# Run the TUI binary
cargo run -p pulsar-tui-bin

# Run the shell binary
cargo run -p pulsar-sh-bin

# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p pulsar-core

# Run a single test by name
cargo test -p pulsar-core test_name

# Check without building
cargo check

# Lint
cargo clippy

# Format
cargo fmt
```

The WASM target (`web/pulsar-wasm`) is compiled separately and is not part of the standard `cargo build` workspace flow — it requires `wasm-pack` or a `wasm32-unknown-unknown` target.

## Workspace Layout

```
pulsar/
├── crates/
│   ├── pulsar-core/    # REPL engine, evaluator, compiler interface (shared library)
│   ├── pulsar-tui/     # Ratatui frontend library
│   └── pulsar-sh/      # Shell runtime library
├── bins/
│   ├── pulsar-tui/     # TUI binary (depends on pulsar-core + pulsar-tui)
│   └── pulsar-sh/      # Shell binary (depends on pulsar-core + pulsar-sh)
└── web/
    └── pulsar-wasm/    # WASM build (depends on pulsar-core, compiled separately)
```

**Key architectural rule:** `pulsar-core` is the shared foundation. All frontends (`pulsar-tui`, `pulsar-sh`, `pulsar-wasm`) depend on it and should not depend on each other. Business logic belongs in `pulsar-core`; frontend-specific rendering/IO belongs in the respective crate.

The binary crates under `bins/` exist solely as entry points — they wire together the library crates and should contain minimal logic.

## Rust Edition

All crates use Rust edition `2024`.
