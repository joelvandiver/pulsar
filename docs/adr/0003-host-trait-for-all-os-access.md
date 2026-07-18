# ADR-0003: All OS access behind a `Host` trait

**Status:** Accepted · 2026-07-18

## Context

Pulsar targets two environments with very different capabilities:

- `pulsar-shell` (native): real filesystem, process spawning, environment
  variables, stdio, clock.
- `pulsar-wasm` (browser): no process spawning, no real filesystem, sandboxed I/O.

If the interpreter calls `std::fs`/`std::process` directly, the wasm build
becomes a fork of the engine with surgery throughout. The "subset of
capabilities" requirement needs a first-class mechanism, not `#[cfg]` scatter.

## Decision

`pulsar-core` is **pure**: it performs no I/O and reads no ambient state. Every
effect goes through a `Host` trait defined in `pulsar-host` (stdio, filesystem,
environment, process spawn, time, random). Hosts report their capabilities;
builtins declare the capability they require. Calling a builtin whose capability
the host lacks returns a typed `CapabilityError` ("`spawn` is not available on
this host"), never a panic.

Implementations:

- `NativeHost` in `pulsar-shell` — full syscall surface via `std`.
- `WasmHost` in `pulsar-wasm` — virtual fs, buffered stdio, no spawn.
- `MockHost` in tests — records calls, serves canned results (M5).

## Enforcement

The **`wasm-check` CI job** compiles `pulsar-core` and `pulsar-syntax` for
`wasm32-unknown-unknown` on every push. Any direct OS dependency sneaking into
the pure crates fails the build. This ADR is not fully accepted until that job
is in `.github/workflows/ci.yml`.

## Consequences

- The wasm "subset" is just a host that says no — same engine, no fork
  (payoff tested in M8: `spawn` under `WasmHost` returns the M5 `CapabilityError`
  with no new mechanism).
- All interpreter logic is testable with `MockHost` — no tempdir/subprocess
  fixtures for core semantics tests.
- Cost: one indirection layer, and builtins must be written against the trait
  even when a direct `std` call would be shorter.
- UIs can query the capability report (`capabilities()` in the M8 JS API) to
  show users what's supported before they hit an error.
