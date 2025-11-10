// src/lib.rs
pub mod engines;
pub mod errors;
pub mod types;

use errors::RedactError;
use types::{Finding, Span};

/// A redactor works on a single string and returns non-overlapping spans + labels.
/// It borrows its compiled pattern set (lifetimes in play).
/// (Compiling regex/AC is relatively expensive; so we want to compile once in main() and lend
/// references to the engines. -- to exercise borrowing and lifetimes)
pub trait Redactor<'p> {
    fn find_all<'a>(&'a self, text: &'a str) -> Vec<Finding>;
    fn label(&self) -> &'static str;
}

/// Merge overlapping/adjacent spans and apply the replacement tokens
pub fn apply_spans(
    mut s: String,
    mut spans: Vec<(Span, &'static str)>,
    token: &str,
) -> Result<(String, usize), RedactError> {
    if spans.is_empty() {
        return Ok((s, 0));
    }

    spans.sort_by_key(|(span, _)| span.start);

    // Merge
    let mut merged: Vec<Span> = Vec::with_capacity(spans.len());
    let mut current = spans[0].0;
    for (span, _) in spans.into_iter().skip(1) {
        if span.start <= current.end {
            // If overlapping or adjacent, extend the current span
            current.end = current.end.max(span.end);
        } else {
            // Otherwise, push the current span and start a new one
            merged.push(current);
            current = span;
        }
    }
    merged.push(current);

    // Apply from the back to preserve indices
    let mut replaced = 0;
    for span in merged.into_iter().rev() {
        if span.start >= span.end || span.end > s.len() {
            return Err(RedactError::BadSpan);
        }
        s.replace_range(span.start..span.end, token);
        replaced += 1;
    }

    Ok((s, replaced))
}

/// Generic processing entrypoint
/// `R` can be RegexRedactor, ACRedactor, etc.
pub fn process<'p, R: Redactor<'p>>(
    engine: &R,
    text: &str,
    token: &str,
) -> Result<(String, usize), RedactError> {
    let spans = engine
        .find_all(text)
        .into_iter()
        .map(|finding| (finding.span, finding.label))
        .collect();
    apply_spans(text.to_string(), spans, token)
}
