# rust-starter

[![CI](https://github.com/swastikjha008-jpg/rust-starter/actions/workflows/ci.yml/badge.svg)](https://github.com/swastikjha008-jpg/rust-starter/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
![Rust](https://img.shields.io/badge/rust-stable-orange.svg)

A small, dependency-free Rust CLI — my first Rust project. It is split into a
reusable library (`src/lib.rs`) and a thin binary (`src/main.rs`), with unit
tests, doc tests, end-to-end CLI tests, and GitHub Actions CI.

## Features

- `greet` — print a friendly greeting
- `fib` — N-th Fibonacci number (overflow-safe, N ≤ 93)
- `prime` — primality check using 6k ± 1 trial division
- `wc` — count lines, words and characters from a file or stdin
- Zero external dependencies

## Getting started

Install Rust via [rustup](https://rustup.rs), then:

```bash
git clone https://github.com/swastikjha008-jpg/rust-starter.git
cd rust-starter
cargo run -- --help
```

## Usage

```bash
cargo run -- greet Ferris          # Hello, Ferris!
cargo run -- fib 50                # 12586269025
cargo run -- prime 97              # 97 is prime
cargo run -- wc README.md          # N lines, N words, N chars
echo "hello rust" | cargo run -- wc -
```

Build an optimized binary:

```bash
cargo build --release
./target/release/rust-starter greet world
```

## Project structure

```
rust-starter/
├── .github/workflows/ci.yml   # fmt + clippy + tests on Linux/macOS/Windows
├── src/
│   ├── lib.rs                 # library: logic, unit tests, doc tests
│   └── main.rs                # CLI entry point
├── tests/
│   └── cli.rs                 # end-to-end tests against the binary
├── Cargo.toml
├── LICENSE
└── README.md
```

## Development

```bash
cargo fmt --all            # format
cargo clippy --all-targets -- -D warnings   # lint
cargo test                 # unit + doc + integration tests
```

## Roadmap

- [ ] Add `clap` for argument parsing
- [ ] Add a `--json` output flag
- [ ] Publish to crates.io

## License

[MIT](LICENSE)
