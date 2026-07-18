# ADR-0004: Macros are special-cased builtins, not expanded

**Status:** Accepted · 2026-07-18

## Context

Idiomatic Rust — and any believable Rust shell session — leans on macros:
`println!`, `format!`, `vec!`. But real macro expansion means implementing
`macro_rules!` matching (and declining proc macros anyway, since they require
compiled plugins). `syn` parses macro invocations only as opaque token streams;
nothing downstream understands them for free.

## Decision

A **fixed set of macros is interpreted as builtins** with format-string support:

`println!`, `print!`, `eprintln!`, `format!`, `vec!`, `assert!`, `assert_eq!`,
`dbg!`

The interpreter recognizes these invocations during lowering/eval and executes
builtin implementations (format-string parsing per M5: `{}`, `{:?}`, positional
args, `{{` escaping, arity errors). Any other macro invocation is a **runtime
error** with a clear message naming the macro and pointing to this policy — not
a panic, not a silent no-op.

## Consequences

- User-defined macros (`macro_rules!`) and proc macros are out of scope for
  M0–M9. Scripts relying on them fail with an explanatory error.
- The builtin set can grow deliberately (e.g. `write!`, `matches!`) — each
  addition follows TDD like any builtin and is recorded in `docs/builtins.md`.
- Format-string behavior is our implementation; divergences from `std::fmt`
  (e.g. unsupported format specs) are documented in `docs/language.md` per
  ADR-0001's no-silent-divergence rule.
- Output macros route through the `Host` (ADR-0003) — `println!` writes to host
  stdout, making it capturable in tests and in the browser.
