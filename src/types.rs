// src/types.rs

/// A half-open byte range (`[star, end)`) indicating the location of a finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

/// A single match plus a static label
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    pub span: Span,
    pub label: &'static str, // e.g., "email
}

/// A summary per file (used by CLI)
#[derive(Debug, Clone, serde::Serialize)]
pub struct Report {
    pub file: String,
    pub replaced: usize,
    pub findings: Vec<(String /*label*/, usize /*count*/)>,
}
