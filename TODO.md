# TODO

## pulsar-core

- [ ] Define core data types: `Expr`, `Value`, `EvalResult`, `PulsarError`
- [ ] Implement expression parser (or integrate a Rust parsing crate)
- [ ] Implement basic expression evaluator (arithmetic, let bindings, type display)
- [ ] Session state: variable bindings, history, context preservation across inputs
- [ ] Multi-line input buffering (detect incomplete expressions)
- [ ] Snippet export to `.rs` file

## pulsar-sh (shell runtime)

- [ ] Basic command parsing and dispatch
- [ ] Process spawning and exit code handling
- [ ] Pipe support (`|`)
- [ ] Redirect support (`>`, `>>`, `<`)
- [ ] Built-in commands: `cd`, `exit`, `history`
- [ ] Signal handling (Ctrl-C, Ctrl-Z, SIGCHLD)
- [ ] Inspectable primitives: expose fd table, process tree, pipe internals as queryable state

## pulsar-tui (terminal UI)

- [ ] Input widget with multi-line editing and history navigation
- [ ] Output pane with syntax-highlighted Rust
- [ ] Error display with pedagogical annotations
- [ ] Step-through evaluation view (stack frames, ownership transitions)
- [ ] Inline documentation panel (hover/query an expression)

## pulsar-wasm (browser build)

- [ ] Configure `wasm-pack` build pipeline
- [ ] Expose `pulsar-core` eval API to JavaScript
- [ ] Minimal browser UI (input + output)

## Error Pedagogy

- [ ] Map common `rustc` error codes to plain-language explanations
- [ ] Borrow checker error explainer (E0382 move, E0502 borrow conflict, etc.)
- [ ] Link errors to relevant curriculum concepts

## Curriculum

- [ ] Lesson format/schema (structured content + embedded exercises)
- [ ] Module: memory and the stack/heap
- [ ] Module: ownership and move semantics
- [ ] Module: borrowing and lifetimes
- [ ] Module: types and traits
- [ ] Module: concurrency primitives
