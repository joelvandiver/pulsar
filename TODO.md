# TODO

Tasks are tracked as GitHub issues. This file is a quick-reference index.

## pulsar-core

- [x] Define core data types: `Expr`, `Value`, `EvalResult`, `PulsarError` — merged in #1
- [ ] Implement expression parser — #2
- [ ] Implement basic expression evaluator — #3
- [ ] Session state: variable bindings, history, context preservation — #4
- [ ] Multi-line input buffering — #5
- [ ] Snippet export to `.rs` file — #6

## pulsar-sh (shell runtime)

- [ ] Basic command parsing and dispatch — #7
- [ ] Process spawning and exit code handling — #8
- [ ] Pipe support (`|`) — #9
- [ ] Redirect support (`>`, `>>`, `<`) — #10
- [ ] Built-in commands: `cd`, `exit`, `history` — #11
- [ ] Signal handling (Ctrl-C, Ctrl-Z, SIGCHLD) — #12
- [ ] Inspectable primitives: fd table, process tree, pipe internals — #13

## pulsar-tui (terminal UI)

- [ ] Input widget with multi-line editing and history — #14
- [ ] Output pane with syntax-highlighted Rust — #15
- [ ] Error display with pedagogical annotations — #16
- [ ] Step-through evaluation view — #17
- [ ] Inline documentation panel — #18

## pulsar-wasm (browser build)

- [ ] Configure `wasm-pack` build pipeline — #19
- [ ] Expose `pulsar-core` eval API to JavaScript — #20
- [ ] Minimal browser UI — #21

## Error Pedagogy

- [ ] Map common `rustc` error codes to plain-language explanations — #22
- [ ] Borrow checker error explainer — #23
- [ ] Link errors to curriculum concepts — #24

## Curriculum

- [ ] Lesson format/schema — #25
- [ ] Module: memory and the stack/heap — #26
- [ ] Module: ownership and move semantics — #27
- [ ] Module: borrowing and lifetimes — #28
- [ ] Module: types and traits — #29
- [ ] Module: concurrency primitives — #30
