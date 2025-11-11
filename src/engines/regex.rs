// src/engines/regex.rs

use crate::Redactor; // lib.rs
use crate::types::{Finding, Span};
use regex::Regex;

/// Regex-based redactor
pub struct RegexRedactor<'p> {
    label: &'static str,
    re: &'p Regex, // borrow compiled regex
}

/// Constructor for RegexRedactor
impl<'p> RegexRedactor<'p> {
    pub fn new(label: &'static str, re: &'p Regex) -> Self {
        Self { label, re }
    }
}

/// Implement Redactor methods for RegexRedactor
impl<'p> Redactor<'p> for RegexRedactor<'p> {
    /// Find all matches in the text and return their spans and labels
    fn find_all<'a>(&'a self, text: &'a str) -> Vec<Finding> {
        self.re
            .find_iter(text)
            .map(|m| Finding {
                span: Span {
                    start: m.start(),
                    end: m.end(),
                },
                label: self.label,
            })
            .collect()
    }

    /// Return the label associated with this redactor
    fn label(&self) -> &'static str {
        self.label
    }
}
