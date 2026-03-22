# PULSAR 🌟

## Crate Organization

```
pulsar/
├── crates/
│   ├── pulsar-core/       # REPL engine, evaluator, compiler interface
│   ├── pulsar-tui/        # Ratatui frontend library
│   └── pulsar-shell/      # Shell runtime library
├── bins/
│   ├── pulsar/            # TUI binary — depends on pulsar-core + pulsar-tui
│   └── pulsar-shell/      # Shell binary — depends on pulsar-core + pulsar-shell
└── web/
    └── pulsar-wasm/       # WASM build — depends on pulsar-core, compiled separately
```

## Features

- **Interactive Rust REPL** — Evaluate Rust expressions and small programs without a full project setup
- **Guided CS curriculum** — Built-in lessons and exercises covering memory, ownership, types, concurrency, and more
- **Integrated shell** — A Rust-powered shell that doubles as a teaching tool for systems concepts
- **Inline documentation** — Hover or query any expression for context-sensitive explanations
- **Step-through evaluation** — Visualize how Rust evaluates expressions, manages the stack, and moves ownership
- **Error pedagogy** — Compiler errors are explained in plain language with links to relevant concepts

---

## Goals

PULSAR is built around a few core beliefs:

1. **Learning by doing beats learning by reading.** Every concept should be immediately interactive.
2. **Rust's strictness is a feature, not a bug.** The borrow checker is a teacher — PULSAR helps you listen to it.
3. **Systems knowledge compounds.** Understanding memory, processes, and I/O makes you a better programmer in any language.

---

## Architecture

PULSAR is composed of two closely related tools:

### `pulsar-repl`

The core REPL for evaluating Rust code interactively. Built on top of the Rust compiler infrastructure, it supports:

- Expression evaluation with type inference display
- Multi-line input with context preservation
- Session history and replay
- Snippet export to `.rs` files

### `pulsar-shell`

A Rust-native shell that teaches systems architecture from the inside out. As you use it, you can inspect what's happening under the hood — process spawning, pipes, file descriptors, signals, and more.

---

## Getting Started

> **Note:** PULSAR is in early development. Installation instructions will be updated as the project matures.

```bash
# Clone the repository
git clone https://github.com/your-username/pulsar
cd pulsar

# Build with Cargo
cargo build --release

# Launch the REPL
./target/release/pulsar-repl

# Or launch the shell
./target/release/pulsar-shell
```

---

## Example Session

```
pulsar> let x: u32 = 42;
pulsar> x * 2
→ 84  (u32)

pulsar> let s = String::from("hello");
pulsar> let t = s;
pulsar> println!("{}", s);
✗ error[E0382]: borrow of moved value: `s`

  💡 PULSAR: When you assigned `s` to `t`, ownership moved.
     `s` is no longer valid. This is Rust's move semantics —
     only one owner at a time. Try cloning: `let t = s.clone();`
```

---

## Roadmap

- [ ] Core REPL with expression evaluation
- [ ] Ownership and borrow checker visualizer
- [ ] Built-in curriculum: memory, types, traits, lifetimes, concurrency
- [ ] `pulsar-shell` with inspectable process primitives
- [ ] WASM build for browser-based learning (no install required)
- [ ] Plugin system for community-authored lessons

---

## Contributing

PULSAR is open to contributors at all experience levels — especially those who are themselves learning Rust. If you've struggled with a concept and figured out a better way to explain it, that knowledge belongs here.

See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

---

## License

MIT — see [LICENSE](./LICENSE) for details.

---

*Named for the precise, rhythmic signals that pulsars emit — a fitting metaphor for a tool that helps you find the steady beat of systems programming.*