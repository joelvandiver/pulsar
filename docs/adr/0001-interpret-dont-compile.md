# ADR-0001: Interpret Rust, don't compile it

**Status:** Accepted · 2026-07-18

## Context

Pulsar's language *is* Rust: users type Rust at a shell prompt or run Rust-syntax
scripts, and Pulsar executes them. Two execution strategies exist:

1. **Compile** — shell out to `rustc` per input (the `evcxr` approach), or embed a
   compiler/`miri`-style engine.
2. **Interpret** — parse to an AST and tree-walk it in-process.

Constraints that decide it:

- `pulsar-wasm` must run the engine in the browser. `rustc` cannot be shipped to
  or invoked from wasm32; any compile-based design makes the wasm milestone a
  rewrite instead of a new host.
- A shell needs sub-100ms feedback per line. `rustc` round-trips cost seconds.
- `miri` embedding was considered and rejected: it rides unstable compiler
  internals and carries the full toolchain's size and churn.

## Decision

Pulsar executes via a **tree-walking interpreter** over its own AST, implemented
in `pulsar-core`, with no dependency on `rustc` at runtime.

## Consequences

- We implement Rust semantics ourselves and support a growing, explicitly
  documented **subset** ("Pulsar Rust"). `docs/language.md` is the contract:
  every divergence from rustc semantics is recorded there — we never silently
  differ.
- Out of scope for the current roadmap (M0–M9): borrow checking, lifetimes,
  generics, traits, real macro expansion (see [ADR-0004](0004-macros-as-builtins.md)).
- Startup and per-line latency are interpreter-bound, not compiler-bound —
  suitable for a REPL.
- The same engine runs everywhere a host exists (see
  [ADR-0003](0003-host-trait-for-all-os-access.md)), which is what makes
  `pulsar-wasm` a milestone rather than a fork.
