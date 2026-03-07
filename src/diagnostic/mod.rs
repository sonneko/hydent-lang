pub mod converter;
pub mod diagnotice_patterns;
pub mod stream;

use crate::compiler::span::Span;

#[derive(Debug)]
pub struct Highlight<'ms> {
    pub span: Span,
    pub label: Option<&'ms str>,
    pub is_primary: bool,
}

#[derive(Debug)]
pub struct Suggestion<'ms> {
    pub message: &'ms str,
    pub replacement_span: Option<Span>,
    pub replacement_text: Option<&'ms str>,
}

pub enum DiagnosticLevel {
    Error,
    Warning,
    Note,
}

pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub highlights: Vec<Highlight<'static>>,
    pub suggestions: Vec<Suggestion<'static>>,
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}
