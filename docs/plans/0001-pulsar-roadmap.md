# Pulsar Roadmap — pulsar-shell → pulsar-wasm

**Status:** Draft for review · **Date:** 2026-07-18 · **Owner:** joelvandiver

## Vision

A shell whose scripting language *is* Rust. You type Rust expressions/statements at a
prompt (or run `.rs`-style scripts) and Pulsar parses and interprets them for
shell-based tasks: full runtime semantics for a practical Rust subset, builtin
functions, and syscalls (files, processes, env). Later, `pulsar-wasm` exposes the
same engine in the browser with the subset of capabilities wasm can support.

## Architectural decisions (proposed ADRs)

### ADR-1: Interpret, don't compile

`evcxr`-style "shell out to rustc" would give perfect Rust semantics but cannot run
in wasm and has multi-second latency per input. Pulsar uses a **tree-walking
interpreter** over its own AST. Trade-off: we implement Rust semantics ourselves and
support a growing *subset* of the language ("Pulsar Rust"), documented explicitly.

### ADR-2: Parse with `syn`, lower to our own AST

`syn` parses the full Rust grammar on stable, is battle-tested, and compiles to
wasm32. We do **not** interpret `syn` types directly; we lower them into a Pulsar
AST (`pulsar-syntax`). That keeps the interpreter decoupled so the parser is
swappable (e.g., a custom parser later for better spans/error recovery), and lets us
attach source spans for diagnostics.

### ADR-3: All OS access behind a `Host` trait

The interpreter core is pure: no `std::fs`, no `std::process`, no clock. Everything
effectful goes through a `Host` trait (fs, spawn, env, stdio, time, random).
`pulsar-shell` provides `NativeHost` (full syscalls); `pulsar-wasm` provides
`WasmHost` (virtual fs, no spawn). Builtins declare required capabilities so the
wasm build fails gracefully ("`spawn` not available on this host") instead of
panicking.

### ADR-4: Macros are special-cased, not expanded

`syn` parses macro invocations as opaque token streams. Real macro expansion is out
of scope. A small set of macros (`println!`, `print!`, `format!`, `eprintln!`,
`vec!`, `assert!`/`assert_eq!`, `dbg!`) are interpreted as builtins with
format-string support. Unknown macros are a runtime error with a clear message.

### Workspace layout

```
pulsar/
├── Cargo.toml            # workspace
├── crates/
│   ├── pulsar-syntax     # source → Pulsar AST (syn-based lowering), spans
│   ├── pulsar-core       # values, environments, evaluator; NO OS deps
│   ├── pulsar-host       # Host trait + capability model (no impls)
│   ├── pulsar-shell      # bin: REPL, scripts, NativeHost, shell builtins
│   └── pulsar-wasm       # cdylib: wasm-bindgen API, WasmHost (subset)
└── docs/
```

Dependency rule (enforced by a CI check, see M0): `pulsar-core` and
`pulsar-syntax` must compile for `wasm32-unknown-unknown`; only `pulsar-shell` may
depend on `std::process`/native-only crates.

---

## Milestones

Every milestone follows Red → Green → Refactor → CI → Document. A task is done only
when its tests exist, pass, CI is green, and the change is documented.

### M0 — Workspace scaffold + CI backbone

The point of doing CI first: every later milestone names checks that must already
exist.

- **Red:** `cargo test --workspace` runs a trivial failing test in each crate
  (`smoke::it_compiles`, asserting a placeholder function's output) — proves the
  test harness wires up per-crate.
- **Green:** workspace `Cargo.toml`, five crate skeletons, placeholder functions
  making smoke tests pass.
- **Refactor:** shared lints (`[workspace.lints]`), MSRV pin, deny `unwrap` in
  non-test code via clippy config.
- **CI:** GitHub Actions workflow with jobs: `fmt` (`cargo fmt --check`), `clippy`
  (`-D warnings`), `test` (`cargo test --workspace`), `wasm-check`
  (`cargo check -p pulsar-core -p pulsar-syntax --target wasm32-unknown-unknown`)
  — this last job *is* the enforcement of ADR-3's dependency rule.
- **Document:** README with vision + layout; ADRs 1–4 as `docs/adr/`.

### M1 — Parse pipeline: source → Pulsar AST

- **Red (unit, in `pulsar-syntax`):**
  - `parse_literal_int`, `parse_literal_str`, `parse_binary_expr` — assert AST shape
    for `1 + 2 * 3` (precedence preserved).
  - `parse_let_binding` — `let x = 5;` → `Stmt::Let { pat, init }`.
  - `parse_error_reports_span` — `let x = ;` yields an error naming line/column, not
    a panic.
  - `parse_repl_fragment` — bare expression `1 + 1` (not a full file) parses; this
    is the REPL entry point and needs its own grammar mode.
- **Green:** `syn` parse of `syn::File` and of a REPL fragment
  (statements-or-expression), lowering into `pulsar_syntax::ast` with spans.
- **Refactor:** snapshot tests (`insta`) for AST dumps to make future grammar
  growth cheap to verify.
- **CI:** covered by `test` + `wasm-check`.
- **Document:** `docs/language.md` started — grammar of the supported subset.

### M2 — Evaluator core: expressions, bindings, scopes

- **Red (unit, in `pulsar-core`):**
  - Happy path: `eval("1 + 2") == Value::Int(3)`, string concat, bool ops,
    comparison operators, unary ops.
  - Bindings/scopes: `let x = 2; x * 3` → 6; shadowing; inner block scope does not
    leak (`{ let y = 1; } y` is an unknown-identifier error).
  - Mutability: `let mut x = 1; x = 2; x` → 2; assignment to non-`mut` is an error
    (Rust semantics, not scripting-language looseness).
  - Error cases: type mismatch (`1 + "a"`), integer overflow behavior (define it:
    wrapping vs error — recommend error, it's a shell), division by zero.
- **Green:** `Value` enum (Int, Float, Bool, Str, Unit…), `Env` with lexical
  scoping, `eval_expr`/`eval_stmt` tree walk.
- **Refactor:** interned identifiers; result-type plumbing for diagnostics.
- **CI:** `test`, `wasm-check`.
- **Document:** language.md — semantics table (what matches rustc, what differs).

### M3 — Control flow and functions

- **Red (unit):**
  - `if`/`else if`/`else` as expressions (`let x = if c { 1 } else { 2 };`).
  - `while`, `loop` + `break`/`continue`, `break` with value from `loop`.
  - `for x in 0..10` over ranges.
  - `fn` items: definition, call, recursion (`fib(10)` → 55), wrong-arity error.
  - Closures: capture by value semantics defined + tested; closures as arguments.
  - `match` on literals, `_`, tuple patterns, binding patterns; non-exhaustive
    match at runtime = error case test.
  - Early `return`; trailing-expression return.
- **Green:** call frames, function values, pattern matcher, loop control via
  interpreter signals (`ControlFlow` enum).
- **Refactor:** recursion-depth guard (stack overflow → clean error) with its own
  test.
- **CI:** `test`, `wasm-check`.
- **Document:** language.md — functions/patterns section.

### M4 — Data: structs, enums, methods, collections

- **Red (unit):**
  - Tuples, arrays/`Vec` (via `vec!`), indexing + out-of-bounds error.
  - `struct` definition, field access, field mutation, missing-field error.
  - `enum` with data-carrying variants; `match` destructuring them; `Option`/
    `Result` built in with `Some/None/Ok/Err` in prelude.
  - `impl` blocks: methods with `self`/`&self`/`&mut self` (define semantics:
    interpreter uses reference-counted values — document the divergence from
    borrow checking), associated functions (`Point::new`).
  - Method resolution error case: unknown method names the type in the message.
  - Core methods on builtins: `String::len/push_str/split`, `Vec::push/len/iter`-
    lite (`map`/`filter`/`collect` over eager iterators).
- **Green:** heap values (`Rc<RefCell<…>>`), method tables per type, prelude
  registration.
- **Refactor:** shared "builtin method registry" so M5 builtins reuse it.
- **CI:** `test`, `wasm-check`.
- **Document:** language.md — data types; explicitly document "no borrow checker:
  runtime is GC-by-refcount" as a known divergence.

### M5 — Host trait, capabilities, builtin functions

- **Red (unit, `pulsar-host` + `pulsar-core`):**
  - `MockHost` (test double) records calls: `println!("hi {}", 1)` writes
    `"hi 1\n"` to mock stdout; `format!` returns without writing.
  - `read_to_string("f")` returns mock file content; missing file → `Err` Value
    (Rust-shaped: builtins return `Result`, tested).
  - Capability gating: a host reporting `spawn: unavailable` makes `spawn(...)`
    return a `CapabilityError` — the exact mechanism pulsar-wasm relies on.
  - Format-string edge cases: `{}` vs `{:?}`, too-few-args error, escaped `{{`.
- **Green:** `Host` trait (stdio, fs, env, spawn, time), capability enum,
  builtin registry bridging interpreter ↔ host, macro-builtins (ADR-4).
- **Refactor:** none anticipated beyond registry cleanup.
- **CI:** `test`, `wasm-check` (MockHost lives behind `cfg(test)`-friendly module
  so core stays wasm-clean).
- **Document:** `docs/builtins.md` — every builtin, signature, required capability.

### M6 — pulsar-shell: REPL, scripts, process execution

This is the first crate where **integration tests** dominate: the boundary is the
real OS.

- **Red:**
  - Unit: line classifier (complete expression vs needs-continuation — `fn f() {`
    should prompt for more input); prompt state machine.
  - Integration (spawn the built binary via `assert_cmd`):
    - `pulsar -c 'println!("hi")'` → stdout `hi\n`, exit 0.
    - Script file execution: `pulsar script.prs` runs, exit code = script's
      `exit(n)` or 0.
    - Runtime error → nonzero exit, diagnostic on stderr with span.
    - `spawn("echo", ["x"])` builtin: captures child stdout, propagates exit
      status; nonexistent command → `Err`, not a panic.
    - `cd("/tmp")` + `cwd()` round-trip; `env("HOME")` reads real env.
    - Stdin piping: `echo '1+1' | pulsar` prints `2`.
  - Integration (REPL, via `rexpect` or pty harness): multi-line input, state
    persists across lines (`let x = 1` then `x + 1` → 2), Ctrl-D exits cleanly.
- **Green:** `NativeHost` (std::fs/process/env), `rustyline` REPL, `-c` flag,
  script mode, exit-code plumbing.
- **Refactor:** extract REPL loop from I/O for unit-testability.
- **CI:** add `integration` job running `cargo test -p pulsar-shell --test '*'` on
  Linux + macOS matrix (process/pty behavior differs per-OS — CI must prove both).
- **Document:** README quickstart; `docs/shell.md` (flags, exit codes, REPL keys).

### M7 — Shell ergonomics: pipelines, sugar for command running

Running external commands must feel shell-like, not ceremony-like.

- **Red:**
  - Unit: design-level tests for the chosen sugar — recommend `sh("ls -la")` →
    `Cmd` value with `.stdout()`, `.lines()`, `.status()`; and `|` composition of
    `Cmd` values (`sh("ls") | sh("wc -l")` pipes at the fd level).
  - Integration: real pipelines produce identical output to `sh -c` equivalents;
    a failing middle command surfaces its status; large-output pipeline (1MB)
    doesn't deadlock (classic pipe-buffer bug — regression test up front).
  - Error cases: `.stdout()` on a failed command → `Err` with stderr attached.
- **Green:** `Cmd` builtin type over `Host::spawn`, fd wiring for pipelines.
- **Refactor:** decide and document glob/interpolation story (`sh(format!(…))`).
- **CI:** integration job (already exists from M6) covers it; add the deadlock
  test with a timeout guard.
- **Document:** `docs/shell.md` — pipeline cookbook comparing bash ↔ pulsar.

### M8 — pulsar-wasm: the subset host

- **Red:**
  - Unit (`pulsar-wasm`, run under `wasm-bindgen-test`): eval `1+1` in wasm;
    `println!` output captured into a JS-visible buffer; `read_to_string` served
    from an in-memory `VirtualFs`; `spawn(...)` returns the M5 `CapabilityError`
    (this test is the payoff of ADR-3 — no new mechanism, just a host that says
    no).
  - Integration: headless-browser test (wasm-pack test --headless --chrome)
    running a small script end-to-end; API surface test for the JS binding
    (`new Pulsar(); p.eval(src) -> {ok, value, stdout}`).
- **Green:** `WasmHost` (virtual fs, buffered stdio, `js_sys` time), wasm-bindgen
  wrapper, capability report API (`p.capabilities()`) so UIs can show what's
  supported.
- **Refactor:** size pass (`wasm-opt`, `panic = "abort"`), budget asserted in CI
  (fail if `.wasm` > agreed size).
- **CI:** new `wasm-test` job (wasm-pack headless chrome) + size-budget check.
- **Document:** `docs/wasm.md` — supported subset table (generated from the
  capability registry, not hand-maintained, so it can't drift), embedding guide.

### M9 — Diagnostics & polish (ongoing, start after M3)

- **Red:** golden-file tests for error output: each diagnostic renders source
  line, caret span, and a help line (`error: unknown method 'puhs' on Vec —
  did you mean 'push'?`).
- **Green:** `miette`-style renderer over the spans carried since M1.
- **CI:** covered by `test`; goldens under `insta`.
- **Document:** contributor guide for adding a diagnostic.

---

## Test scope checklist (whole roadmap)

- [x] Unit: every evaluator/parser feature lands with happy-path, edge, and error
  tests (M1–M5, M9)
- [x] Integration: OS boundary (M6–M7 via `assert_cmd`/pty), browser boundary
  (M8 via wasm-pack headless)
- [x] Regression: deadlock test (M7), recursion-depth (M3), overflow semantics (M2)
- [x] CI: fmt, clippy, test, wasm-check from M0; OS-matrix integration from M6;
  wasm-test + size budget from M8

## Known risks / open questions

1. **Semantics divergence** — no borrow checker, refcounted values. Mitigation:
   document divergences in `language.md` as they're decided; never silently differ.
2. **Scope creep on "full Rust"** — traits, generics, lifetimes are *not* in
   M1–M8. Recommend deferring generics/traits to a post-M9 milestone and being
   loud in docs about the subset. (Flagging now so it's a decision, not a drift.)
3. **REPL/pty tests are flaky-prone in CI** — mitigate with generous timeouts and
   retry-once policy; flag any test that needs `#[ignore]` as a gap, per the
   working agreement.
4. **Manual-only verification gaps** — browser demo look-and-feel (M8) is the one
   thing CI can't fully prove; the headless eval tests close the functional part,
   the visual part remains a documented manual step.
