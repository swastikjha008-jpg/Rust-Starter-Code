//! End-to-end tests that run the compiled binary.

use std::io::Write;
use std::process::{Command, Output, Stdio};

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_rust-starter"))
}

fn run(args: &[&str]) -> Output {
    bin().args(args).output().expect("failed to run binary")
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

#[test]
fn greet_with_name() {
    let out = run(&["greet", "Ferris", "the", "Crab"]);
    assert!(out.status.success());
    assert_eq!(stdout(&out), "Hello, Ferris the Crab!");
}

#[test]
fn greet_without_name() {
    let out = run(&["greet"]);
    assert!(out.status.success());
    assert_eq!(stdout(&out), "Hello, world!");
}

#[test]
fn fib_works() {
    let out = run(&["fib", "50"]);
    assert!(out.status.success());
    assert_eq!(stdout(&out), "12586269025");
}

#[test]
fn fib_overflow_is_an_error() {
    let out = run(&["fib", "94"]);
    assert!(!out.status.success());
}

#[test]
fn prime_works() {
    assert_eq!(stdout(&run(&["prime", "97"])), "97 is prime");
    assert_eq!(stdout(&run(&["prime", "100"])), "100 is not prime");
}

#[test]
fn invalid_number_fails() {
    let out = run(&["prime", "abc"]);
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("invalid number"));
}

#[test]
fn unknown_command_fails() {
    let out = run(&["nope"]);
    assert!(!out.status.success());
}

#[test]
fn help_and_version() {
    assert!(stdout(&run(&["--help"])).contains("USAGE"));
    assert!(stdout(&run(&[])).contains("USAGE"));
    assert!(stdout(&run(&["--version"])).starts_with("rust-starter "));
}

#[test]
fn wc_reads_stdin() {
    let mut child = bin()
        .args(["wc", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn binary");
    child
        .stdin
        .take()
        .expect("stdin not piped")
        .write_all(b"hello rust\nworld\n")
        .expect("failed to write stdin");
    let out = child.wait_with_output().expect("failed to wait");
    assert!(out.status.success());
    assert_eq!(stdout(&out), "2 lines, 3 words, 17 chars");
}

#[test]
fn wc_reads_file() {
    let path = std::env::temp_dir().join(format!("rust-starter-test-{}.txt", std::process::id()));
    std::fs::write(&path, "a b c\n").expect("failed to write temp file");
    let out = run(&["wc", path.to_str().expect("non-utf8 path")]);
    std::fs::remove_file(&path).ok();
    assert!(out.status.success());
    assert_eq!(stdout(&out), "1 lines, 3 words, 6 chars");
}

#[test]
fn wc_missing_file_fails() {
    let out = run(&["wc", "definitely-not-a-real-file.txt"]);
    assert!(!out.status.success());
}
