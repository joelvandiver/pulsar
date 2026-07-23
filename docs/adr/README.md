# Architecture Decision Records

Decisions are numbered, immutable once accepted (supersede, don't rewrite), and
follow Status / Context / Decision / Consequences.

| ADR | Decision | Status |
|-----|----------|--------|
| [0001](0001-interpret-dont-compile.md) | Interpret Rust with a tree-walking interpreter; never invoke `rustc` at runtime | Accepted |
| [0002](0002-parse-with-syn-lower-to-own-ast.md) | Parse with `syn`, lower to a Pulsar-owned AST with spans | Accepted |
| [0003](0003-host-trait-for-all-os-access.md) | All OS access behind a `Host` trait; capability-gated builtins | Accepted |
| [0004](0004-macros-as-builtins.md) | Fixed macro set interpreted as builtins; no macro expansion | Accepted |
| [0005](0005-target-platforms.md) | Platforms are `Host` impls by capability profile; browser wasm is the first host built | Accepted |
