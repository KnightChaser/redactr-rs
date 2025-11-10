// src/engines/aho.rs
use crate::Redactor;
use crate::types::{Finding, Span};
use aho_corasick::{AhoCorasick, Match};

pub struct ACRedactor<'p> {
    label: &'static str,
    ac: &'p AhoCorasick, // borrow compiled automation
}

impl<'p> ACRedactor<'p> {
    pub fn new(label: &'static str, ac: &'p AhoCorasick) -> Self {
        Self { label, ac }
    }
}

impl<'p> Redactor<'p> for ACRedactor<'p> {
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

    fn label(&self) -> &'static str {
        self.label
    }
}
