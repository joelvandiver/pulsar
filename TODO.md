# Pulsar TODO

Task breakdown of [docs/plans/0001-pulsar-roadmap.md](docs/plans/0001-pulsar-roadmap.md).
Rules: work top-down unless noted; a task is **done** only when tests written →
tests pass → CI green → change documented. Each ☐ lists its Red step first.

## M0 — Scaffold + CI

- [ ] Decide + record ADRs 1–4 in `docs/adr/` (interpret-don't-compile, syn→own
      AST, Host trait, macro special-casing)
- [ ] Red: per-crate `smoke::it_compiles` failing tests · Green: workspace +
      5 crate skeletons (`pulsar-syntax`, `-core`, `-host`, `-shell`, `-wasm`)
- [ ] CI workflow: `fmt`, `clippy -D warnings`, `test --workspace`,
      `wasm-check` (core+syntax on wasm32 target — enforces ADR-3)
- [ ] Workspace lints, MSRV pin, README with vision + layout

## M1 — Parse pipeline

- [ ] Red: literal/binary-precedence AST tests · Green: syn → Pulsar AST lowering
      for expressions, with spans
- [ ] Red: `let` statement test · Green: statement lowering
- [ ] Red: REPL-fragment test (bare `1 + 1` parses) · Green: fragment parse mode
- [ ] Red: parse-error-has-span test · Green: error type with line/col
- [ ] Refactor: `insta` snapshot tests for AST dumps
- [ ] Document: start `docs/language.md` (grammar subset)

## M2 — Evaluator core

- [ ] Red: arithmetic/bool/string/comparison eval tests · Green: `Value`,
      `eval_expr`
- [ ] Red: let/shadowing/block-scope-leak tests · Green: `Env` lexical scoping
- [ ] Red: `mut` assignment ok + non-`mut` assignment error tests · Green:
      mutability tracking
- [ ] Red: type-mismatch, div-by-zero, overflow tests (decide overflow = error)
      · Green: runtime error type
- [ ] Document: semantics table in `language.md`

## M3 — Control flow + functions

- [ ] Red: if-as-expression tests · Green: if/else eval
- [ ] Red: while/loop/break-with-value/continue tests · Green: loop signals
- [ ] Red: `for x in 0..10` test · Green: range iteration
- [ ] Red: fn definition/call/recursion/arity-error tests · Green: call frames
- [ ] Red: closure capture tests · Green: closure values
- [ ] Red: match (literal, `_`, tuple, binding, non-exhaustive-error) tests ·
      Green: pattern matcher
- [ ] Red: recursion-depth-limit test · Green: depth guard with clean error
- [ ] Document: functions/patterns in `language.md`

## M4 — Data types

- [ ] Red: tuple/array/`vec!`/index + OOB-error tests · Green: sequence values
- [ ] Red: struct define/access/mutate/missing-field tests · Green: struct values
- [ ] Red: enum + destructuring match tests; `Option`/`Result` prelude tests ·
      Green: variant values, prelude
- [ ] Red: impl-block method + associated-fn + unknown-method-error tests ·
      Green: method tables (`Rc<RefCell>` heap values)
- [ ] Red: String/Vec core-method tests (incl. `map`/`filter`/`collect` eager
      iterators) · Green: builtin method registry
- [ ] Document: data types + "no borrow checker" divergence note

## M5 — Host trait + builtins

- [ ] Red: `MockHost` println!/format!/eprintln! tests (incl. `{:?}`, arg-count
      error, `{{` escape) · Green: `Host` trait + macro-builtins
- [ ] Red: fs builtin tests via MockHost (read/write/missing-file → `Err`) ·
      Green: fs surface
- [ ] Red: capability-gating test (`spawn` unavailable → `CapabilityError`) ·
      Green: capability enum + registry — **pulsar-wasm depends on this**
- [ ] Document: `docs/builtins.md` (signature + required capability per builtin)

## M6 — pulsar-shell binary

- [ ] Red: line-classifier unit tests (complete vs needs-continuation) · Green:
      classifier
- [ ] Red: `assert_cmd` integration — `-c` flag, script file, exit codes, error
      → stderr+nonzero · Green: `NativeHost` + CLI
- [ ] Red: spawn/cd/cwd/env integration tests · Green: process+env syscalls
- [ ] Red: stdin-pipe test (`echo '1+1' | pulsar` → `2`) · Green: stdin mode
- [ ] Red: pty REPL tests (state across lines, Ctrl-D) · Green: rustyline REPL
- [ ] CI: add `integration` job, Linux+macOS matrix
- [ ] Document: README quickstart, `docs/shell.md`

## M7 — Pipelines + command sugar

- [ ] Red: `sh("...")` → `Cmd` API tests (`.stdout()`, `.lines()`, `.status()`,
      failed-command → `Err` with stderr) · Green: `Cmd` builtin over Host::spawn
- [ ] Red: `Cmd | Cmd` fd-level pipeline integration tests, incl. 1MB no-deadlock
      regression (timeout-guarded) · Green: pipeline wiring
- [ ] Decide + document interpolation/glob story
- [ ] Document: bash ↔ pulsar cookbook in `docs/shell.md`

## M8 — pulsar-wasm

- [ ] Red: wasm-bindgen-test evals (`1+1`, println!-to-buffer, VirtualFs read,
      spawn → `CapabilityError`) · Green: `WasmHost`
- [ ] Red: JS API surface test (`new Pulsar()`, `eval` → `{ok, value, stdout}`,
      `capabilities()`) · Green: wasm-bindgen wrapper
- [ ] Red: headless-chrome end-to-end script test · Green: wasm-pack build
- [ ] CI: `wasm-test` job + `.wasm` size-budget check
- [ ] Document: `docs/wasm.md` — capability table generated from registry

## M9 — Diagnostics (start any time after M3)

- [ ] Red: golden-file diagnostic tests (source line, caret, did-you-mean help) ·
      Green: miette-style renderer over M1 spans
- [ ] Document: contributor guide for adding diagnostics

## Deferred (explicitly out of scope for M0–M9)

- Generics, traits, lifetimes, borrow checking
- Real macro expansion (beyond the ADR-4 builtin set)
- Job control (`&`, `fg`/`bg`), signal handling in REPL
- WASI-based fs for pulsar-wasm (VirtualFs first)
