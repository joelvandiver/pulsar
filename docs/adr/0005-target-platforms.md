# ADR-0005: Target platforms are `Host` implementations, selected by capability profile

**Status:** Accepted · 2026-07-23

## Context

Pulsar's engine (`pulsar-core`) is pure and OS-free; every effect goes through the
`Host` trait ([ADR-0003](0003-host-trait-for-all-os-access.md)). That means a
"target platform" for Pulsar is not primarily an OS — it is a **`Host`
implementation plus the set of capabilities that platform can honor**. The same
interpreter runs anywhere a `Host` exists; what differs per platform is which
capabilities (`fs`, `spawn`, `env`, `stdio`, `time`, `random`) are available and
how they are backed.

We need to (a) name the platforms we intend to support, (b) make the differences
between them explicit rather than discovered ad hoc, and (c) decide which to build
first, given the project's two goals: **learn Rust by building it** and **explore
Rust at runtime** (see the roadmap Vision).

A common confusion to head off: `wasm32-unknown-unknown` (browser) and
`wasm32-wasip1`/`wasip2` (WASI) are **different targets with different hosts** —
browser wasm has no syscalls; WASI standardizes real fs/stdio/env. They are two
platforms, not one.

## Decision

Model each target platform as a distinct `Host` implementation, defined by an
explicit capability profile. The committed set:

| Platform | Rust target triple | fs | spawn | env | stdio | time / rng | Host impl |
|---|---|---|---|---|---|---|---|
| Native Linux / macOS | `*-unknown-linux-gnu`, `*-apple-darwin` | real | ✓ | real | real tty | real | `NativeHost` |
| Native Windows | `*-pc-windows-msvc` | real | ✓ (no `fork`) | real | real | real | `NativeHost` + `cfg` |
| **Browser wasm** | `wasm32-unknown-unknown` | `VirtualFs` | ✗ → `CapabilityError` | virtual | buffer → JS | `js_sys` | `WasmHost` |
| WASI | `wasm32-wasip1` / `wasip2` | real (preopened) | limited / ✗ | real | real | WASI clocks | `WasiHost` (future) |
| Embedded library | host app's triple | host-defined | host-defined | host-defined | host-defined | host-defined | caller's `Host` |

A capability a platform cannot honor is not a panic and not a compile error in the
core — it is a runtime `CapabilityError` from the M5 capability registry. That
mechanism, not per-platform `#[cfg]` in the interpreter, is how targets differ.

**Prioritization:** beyond the native baseline used for day-to-day development, the
**browser wasm target (`wasm32-unknown-unknown`, `WasmHost`) is the first host we
build to completion** — ahead of hardening the native shell (M6) and before WASM's
other cousins. Rationale: a zero-install, shareable browser REPL is the single best
vehicle for "explore Rust at runtime," and the `WasmHost` is the sharpest test of
the capability model (it is the host that must say "no" to `spawn`). WASI, Windows,
and library-embedding are explicitly deferred, not rejected.

## Consequences

- **Dependency reality:** a host is only useful once the core it drives exists.
  `WasmHost` still sits on top of M1–M5 (parse → eval → data → `Host` trait +
  capabilities). "wasm first" reorders *which host ships first*, not the core work
  underneath it; the `wasm-check` CI job already keeps `pulsar-core`/`pulsar-syntax`
  wasm-clean so this stays a new host, never a rewrite.
- **CI grows per platform:** each target is "a `Host` impl + its own CI proof"
  (`rustup target add …` plus a job). Browser wasm adds the M8 `wasm-test`
  (wasm-pack headless) + `.wasm` size-budget jobs; WASI/Windows would add their own.
- **The capability matrix is the contract** and must not drift. The wasm docs
  (`docs/wasm.md`) generate their supported-capability table from the registry, not
  by hand, so the table cannot lie about what a platform allows.
- **WASI is a separate future host**, not a change to `WasmHost`. When added it is a
  new implementation with its own capability profile (real fs via preopened dirs),
  superseding nothing here.
- **Embedding is a first-class shape:** because a platform is just a `Host`, a
  third-party Rust app embedding `pulsar-core` with its own `Host` is a supported
  target, and API design for that case is on the roadmap's later horizon.
- Supersedes nothing; extends [ADR-0003](0003-host-trait-for-all-os-access.md) by
  enumerating the concrete hosts its trait was designed to enable.
