# ADR-0002: Parse with `syn`, lower to our own AST

**Status:** Accepted · 2026-07-18

## Context

Given ADR-0001 (interpret), Pulsar needs a Rust parser. Options:

1. **Hand-rolled lexer/parser** — full control over spans, error recovery, and
   REPL-fragment grammar, but months of work before the first expression evaluates.
2. **Interpret `syn` types directly** — fastest start, but welds the interpreter
   to a parsing library's API surface; every `syn` major bump or future parser
   swap touches the evaluator.
3. **Parse with `syn`, lower into a Pulsar-owned AST** — `syn` handles the full
   Rust grammar on stable, is battle-tested, and compiles cleanly to
   wasm32-unknown-unknown.

## Decision

`pulsar-syntax` parses source with **`syn`** (both whole files and REPL fragments —
a statement-or-expression entry point) and **lowers into a Pulsar-owned AST**
carrying source spans. `pulsar-core` consumes only the Pulsar AST and never sees
`syn` types.

## Consequences

- The parser is swappable: a future hand-rolled parser (better error recovery,
  incremental REPL parsing) replaces only the lowering layer, not the evaluator.
- We maintain a lowering layer, and spans must survive lowering — the
  parse-error and diagnostics tests (M1, M9) pin this.
- Grammar coverage grows by extending the lowering: `syn` already parses
  constructs we don't yet interpret; unlowered constructs must produce a clear
  "not yet supported" error, not a panic.
- `syn`'s span fidelity depends on `proc-macro2`'s fallback mode outside proc
  macros — line/column info is available, byte-offset math is ours to manage in
  the lowering.
