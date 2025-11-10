// src/engines/regex.rs

use crate::Redactor; // lib.rs
use crate::types::{Finding, Span};
use regex::Regex;

pub struct RegexRedactor<'p> {
    label: &'static str,
    re: &'p Regex, // borrow compiled regex
}

impl<'p> RegexRedactor<'p> {
    pub fn new(label: &'static str, re: &'p Regex) -> Self {
        Self { label, re }
    }
}

impl<'p> Redactor<'p> for RegexRedactor<'p> {
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

    fn label(&self) -> &'static str {
        self.label
    }
}
