// src/engines/aho.rs
use crate::Redactor;
use crate::types::{Finding, Span};
use aho_corasick::{AhoCorasick, Match};

/// Aho-Corasick based redactor
pub struct ACRedactor<'p> {
    label: &'static str,
    ac: &'p AhoCorasick, // borrow compiled automation
}

/// Constructor for ACRedactor
impl<'p> ACRedactor<'p> {
    pub fn new(label: &'static str, ac: &'p AhoCorasick) -> Self {
        Self { label, ac }
    }
}

/// Implement Redactor methods for ACRedactor
impl<'p> Redactor<'p> for ACRedactor<'p> {
    /// Find all matches in the text and return their spans and labels
    fn find_all<'a>(&'a self, text: &'a str) -> Vec<Finding> {
        self.ac
            .find_iter(text)
            .map(|m: Match| Finding {
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
