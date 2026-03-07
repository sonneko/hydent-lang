pub mod converter;
pub mod diagnotice_patterns;
pub mod stream;

use std::fmt::Display;

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

impl Display for DiagnosticLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiagnosticLevel::Error => write!(f, "error"),
            DiagnosticLevel::Warning => write!(f, "warning"),
            DiagnosticLevel::Note => write!(f, "note"),
        }
    }
}

pub struct Diagnostic {
    pub id: u32,
    pub message: &'static str,
    pub primary: Span,
    pub level: DiagnosticLevel,
    pub highlights: Vec<Highlight<'static>>,
    pub suggestions: Vec<Suggestion<'static>>,
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //         write!(f,
        // r#"{}[E{}]: {}
        //  -->

        // "#, self.level, self.id, self.message)
        Ok(())
    }
}

impl Default for Diagnostic {
    fn default() -> Self {
        Self {
            id: 0,
            message: "unimplemented",
            primary: Span::new(0, 0),
            level: DiagnosticLevel::Error,
            highlights: Vec::new(),
            suggestions: Vec::new(),
        }
    }
}
