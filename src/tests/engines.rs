// tests/engines.rs
use redactr_rs::{engines::regex::RegexRedactor, process};
use regex::Regex;

#[test]
fn regex_email_basic() {
    let re = Regex::new(r"(?i)[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}").unwrap();
    let eng = RegexRedactor::new("email", &re);
    let input = "a@x.io and B.C+d@Y.CO";
    let (out, n) = process(&eng, input, "[X]").unwrap();

    assert_eq!(n, 2);
    assert!(out.contains("[X] and [X]"));
}
