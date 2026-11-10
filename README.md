<div align="center">

# 🦀 rust-starter

**A tiny, dependency-free Rust CLI — clean structure, real tests, and CI out of the box.**

[![CI](https://img.shields.io/github/actions/workflow/status/swastikjha008-jpg/rust-starter/ci.yml?branch=main&style=for-the-badge&logo=githubactions&logoColor=white&label=CI)](https://github.com/swastikjha008-jpg/rust-starter/actions/workflows/ci.yml)
[![Rust](https://img.shields.io/badge/Rust-stable-F74C00?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-22c55e?style=for-the-badge)](LICENSE)

![Tests](https://img.shields.io/badge/tests-25%20passing-22c55e?style=flat-square&logo=checkmarx&logoColor=white)
![Dependencies](https://img.shields.io/badge/dependencies-0-8b5cf6?style=flat-square)
![Clippy](https://img.shields.io/badge/clippy-clean-0ea5e9?style=flat-square&logo=rust&logoColor=white)
![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20macOS%20%7C%20Windows-64748b?style=flat-square)
![PRs Welcome](https://img.shields.io/badge/PRs-welcome-f59e0b?style=flat-square)

[Features](#-features) •
[Quick Start](#-quick-start) •
[Usage](#-usage) •
[Architecture](#-architecture) •
[Testing](#-testing)

</div>

---

## ✨ About

`rust-starter` is a small command-line tool built as a **first Rust project** and a **reusable template** for the next one. It shows how a well-organised Rust crate looks: logic in a library, a thin binary on top, three layers of tests, strict linting, and automated CI on every push.

No external crates. Just the standard library.

## 🚀 Features

| Command | What it does | Example |
| --- | --- | --- |
| `greet [NAME...]` | Prints a friendly greeting | `greet Ferris` → `Hello, Ferris!` |
| `fib <N>` | N-th Fibonacci number, overflow-safe (N ≤ 93) | `fib 50` → `12586269025` |
| `prime <N>` | Primality check using 6k ± 1 trial division | `prime 97` → `97 is prime` |
| `wc <FILE\|->` | Counts lines, words and chars from a file or stdin | `wc -` → `2 lines, 3 words, 17 chars` |

**Under the hood**

- 🧩 **Library + binary split** — all logic lives in `lib.rs`, so it is easy to test and reuse
- 🛡️ **No panics on bad input** — errors go to `stderr` with a non-zero exit code
- 🔢 **Overflow-safe math** — `fib` returns `None` instead of wrapping silently
- 🧪 **Unit, doc and end-to-end tests** — 25 in total
- 🤖 **GitHub Actions CI** — format, lint and test on Linux, macOS and Windows

## ⚡ Quick Start

### Prerequisites

- [Rust](https://rustup.rs) (stable, recent). Run `rustup update` if the build complains about `is_multiple_of`.
- **Windows only:** [Build Tools for Visual Studio](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with the **Desktop development with C++** workload (Rust needs the MSVC linker).

### Run it

````bash
git clone https://github.com/swastikjha008-jpg/rust-starter.git
cd rust-starter
cargo run -- --help
````

### Build an optimized binary

````bash
cargo build --release
./target/release/rust-starter greet world
````

On Windows the binary is `target\release\rust-starter.exe`.

## 📖 Usage

````text
USAGE:
    rust-starter <COMMAND> [ARGS]

COMMANDS:
    greet [NAME...]   Print a greeting
    fib <N>           Print the N-th Fibonacci number (N <= 93)
    prime <N>         Check whether N is prime
    wc <FILE|->       Count lines, words and chars (use - for stdin)
    help              Show this message

OPTIONS:
    -h, --help        Show this message
    -V, --version     Show version
````

### Examples

````bash
$ cargo run -q -- greet Ferris the Crab
Hello, Ferris the Crab!

$ cargo run -q -- fib 93
12200160415121876738

$ cargo run -q -- prime 7919
7919 is prime

$ printf 'hello rust\nworld\n' | cargo run -q -- wc -
2 lines, 3 words, 17 chars
````

### Errors are friendly

````bash
$ cargo run -q -- fib 94
error: result does not fit in u64 (N must be <= 93)

Run `rust-starter --help` for usage.
````

The process exits with code `1` on any error, so it is safe to use in scripts.

## 🏗️ Architecture

````mermaid
flowchart LR
    A[CLI arguments] --> B["main.rs<br/>parse and dispatch"]
    B --> C["lib.rs<br/>greet · fibonacci · is_prime · count_text"]
    C --> D[stdout]
    B -. error .-> E["stderr<br/>exit code 1"]
````

````text
rust-starter/
├── .github/workflows/ci.yml   # fmt + clippy + tests on Linux / macOS / Windows
├── src/
│   ├── lib.rs                 # core logic, unit tests, doc tests
│   └── main.rs                # CLI entry point (argument parsing, I/O)
├── tests/
│   └── cli.rs                 # end-to-end tests against the compiled binary
├── Cargo.toml
├── Cargo.lock
├── LICENSE
└── README.md
````

### Library API

Use it as a crate from your own code:

| Item | Signature | Notes |
| --- | --- | --- |
| `greet` | `fn greet(name: &str) -> String` | Falls back to "world" for blank names |
| `fibonacci` | `fn fibonacci(n: u32) -> Option<u64>` | `None` when `n > 93` |
| `is_prime` | `fn is_prime(n: u64) -> bool` | Works across the full `u64` range |
| `count_text` | `fn count_text(text: &str) -> Counts` | Returns lines, words and chars |
| `Counts` | `struct { lines, words, chars }` | Implements `Display`, `Debug`, `Default` |

## 🧪 Testing

````bash
cargo test
````

| Layer | Count | Where |
| --- | :---: | --- |
| Unit tests | 10 | `src/lib.rs` |
| End-to-end CLI tests | 11 | `tests/cli.rs` |
| Doc tests | 4 | examples in the `lib.rs` doc comments |

Before committing, run the same checks CI does:

````bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
````

## 📚 What this project demonstrates

- Cargo workspaces basics: library and binary in one package
- Error handling with `Result`, `Option` and the `?` operator
- Pattern matching, slices and string handling
- Implementing a trait (`Display`) for a custom type
- Reading from files and stdin
- Writing unit, doc and integration tests
- Release profile tuning (`lto`, `strip`) and CI with GitHub Actions

## 🤝 Contributing

Contributions, issues and ideas are welcome.

1. Fork the repo and create a branch: `git checkout -b feature/my-idea`
2. Make your change and run the checks above
3. Commit and open a pull request

## 📄 License

Released under the [MIT License](LICENSE).

---

<div align="center">

Built with 🦀 by [**@swastikjha008-jpg**](https://github.com/swastikjha008-jpg)

If this helped you start with Rust, a ⭐ is appreciated.

</div>
````
````

The box uses four backticks on the outside because the README contains its own code blocks. Use the copy button on the box and the inner blocks will paste correctly. The README in the project folder and the zip is updated too, so everything matches.
