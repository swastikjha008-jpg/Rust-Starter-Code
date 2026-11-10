//! # rust-starter
//!
//! A tiny library powering the `rust-starter` CLI. It demonstrates the basics
//! of a well-structured Rust crate: pure functions, doc tests, unit tests,
//! and a small data type with a [`Display`](std::fmt::Display) implementation.

use std::fmt;

/// Builds a greeting for `name`. Falls back to "world" when `name` is blank.
///
/// ```
/// use rust_starter::greet;
///
/// assert_eq!(greet("Ferris"), "Hello, Ferris!");
/// assert_eq!(greet("   "), "Hello, world!");
/// ```
pub fn greet(name: &str) -> String {
    let name = name.trim();
    if name.is_empty() {
        "Hello, world!".to_string()
    } else {
        format!("Hello, {name}!")
    }
}

/// Returns the `n`-th Fibonacci number (0-indexed), or `None` if it does not
/// fit in a `u64` (that is, for `n > 93`).
///
/// ```
/// use rust_starter::fibonacci;
///
/// assert_eq!(fibonacci(0), Some(0));
/// assert_eq!(fibonacci(10), Some(55));
/// assert_eq!(fibonacci(94), None);
/// ```
pub fn fibonacci(n: u32) -> Option<u64> {
    if n > 93 {
        return None;
    }
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 0..n {
        // `b` is unused after the final iteration, so wrapping there is harmless.
        let next = a.wrapping_add(b);
        a = b;
        b = next;
    }
    Some(a)
}

/// Returns `true` if `n` is a prime number (trial division by 6k ± 1).
///
/// ```
/// use rust_starter::is_prime;
///
/// assert!(is_prime(97));
/// assert!(!is_prime(1));
/// assert!(!is_prime(100));
/// ```
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n < 4 {
        return true;
    }
    if n.is_multiple_of(2) || n.is_multiple_of(3) {
        return false;
    }
    let mut i = 5u64;
    while i.saturating_mul(i) <= n {
        if n.is_multiple_of(i) || n.is_multiple_of(i + 2) {
            return false;
        }
        i += 6;
    }
    true
}

/// Line, word, and character totals for a piece of text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Counts {
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
}

impl fmt::Display for Counts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} lines, {} words, {} chars",
            self.lines, self.words, self.chars
        )
    }
}

/// Counts lines, words, and characters in `text`.
///
/// ```
/// use rust_starter::count_text;
///
/// let c = count_text("hello rust\nworld");
/// assert_eq!((c.lines, c.words, c.chars), (2, 3, 16));
/// ```
pub fn count_text(text: &str) -> Counts {
    Counts {
        lines: text.lines().count(),
        words: text.split_whitespace().count(),
        chars: text.chars().count(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_uses_name() {
        assert_eq!(greet("Swastik"), "Hello, Swastik!");
    }

    #[test]
    fn greet_trims_and_defaults() {
        assert_eq!(greet("  Ana  "), "Hello, Ana!");
        assert_eq!(greet(""), "Hello, world!");
    }

    #[test]
    fn fibonacci_small_values() {
        let expected = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
        for (n, want) in expected.iter().enumerate() {
            assert_eq!(fibonacci(n as u32), Some(*want));
        }
    }

    #[test]
    fn fibonacci_limits() {
        assert_eq!(fibonacci(93), Some(12_200_160_415_121_876_738));
        assert_eq!(fibonacci(94), None);
    }

    #[test]
    fn primes() {
        let primes = [2, 3, 5, 7, 11, 13, 97, 7919, 1_000_000_007];
        for p in primes {
            assert!(is_prime(p), "{p} should be prime");
        }
    }

    #[test]
    fn non_primes() {
        let composites = [0, 1, 4, 9, 15, 100, 7917, 1_000_000_007 * 3];
        for c in composites {
            assert!(!is_prime(c), "{c} should not be prime");
        }
    }

    #[test]
    fn largest_u64_prime_does_not_overflow() {
        assert!(is_prime(18_446_744_073_709_551_557));
    }

    #[test]
    fn counts_text() {
        let c = count_text("one two\nthree\n");
        assert_eq!(
            c,
            Counts {
                lines: 2,
                words: 3,
                chars: 14
            }
        );
    }

    #[test]
    fn counts_empty() {
        assert_eq!(count_text(""), Counts::default());
    }

    #[test]
    fn counts_display() {
        let c = count_text("a b");
        assert_eq!(c.to_string(), "1 lines, 2 words, 3 chars");
    }
}
