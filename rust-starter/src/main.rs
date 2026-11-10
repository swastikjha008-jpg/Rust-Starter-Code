use std::io::{self, Read};
use std::process::ExitCode;
use std::{env, fs};

use rust_starter::{count_text, fibonacci, greet, is_prime};

const USAGE: &str = "\
rust-starter - a tiny Rust CLI starter

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
    -V, --version     Show version";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match run(&args) {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("error: {message}\n\nRun `rust-starter --help` for usage.");
            ExitCode::FAILURE
        }
    }
}

fn run(args: &[String]) -> Result<String, String> {
    let rest = args.get(1..).unwrap_or_default();
    match args.first().map(String::as_str) {
        None | Some("help" | "-h" | "--help") => Ok(USAGE.to_string()),
        Some("-V" | "--version") => Ok(format!(
            "{} {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        )),
        Some("greet") => Ok(greet(&rest.join(" "))),
        Some("fib") => {
            let n: u32 = parse_number(rest.first(), "fib <N>")?;
            fibonacci(n)
                .map(|value| value.to_string())
                .ok_or_else(|| "result does not fit in u64 (N must be <= 93)".to_string())
        }
        Some("prime") => {
            let n: u64 = parse_number(rest.first(), "prime <N>")?;
            let verdict = if is_prime(n) { "prime" } else { "not prime" };
            Ok(format!("{n} is {verdict}"))
        }
        Some("wc") => {
            let source = rest
                .first()
                .ok_or_else(|| "missing argument: wc <FILE|->".to_string())?;
            read_input(source).map(|text| count_text(&text).to_string())
        }
        Some(other) => Err(format!("unknown command: {other}")),
    }
}

fn parse_number<T: std::str::FromStr>(arg: Option<&String>, usage: &str) -> Result<T, String> {
    let raw = arg.ok_or_else(|| format!("missing argument: {usage}"))?;
    raw.parse()
        .map_err(|_| format!("invalid number: {raw} (usage: {usage})"))
}

fn read_input(source: &str) -> Result<String, String> {
    if source == "-" {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .map_err(|e| format!("failed to read stdin: {e}"))?;
        Ok(buffer)
    } else {
        fs::read_to_string(source).map_err(|e| format!("failed to read {source}: {e}"))
    }
}
