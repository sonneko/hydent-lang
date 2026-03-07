use crate::diagnostic::{converter::IntoDiagnostic, Diagnostic};

pub trait DiagnosticStream {
    fn pour<D: IntoDiagnostic>(&mut self, diagnostic: D);
}

pub struct InstantStdioDiagnosticStream;

impl DiagnosticStream for InstantStdioDiagnosticStream {
    fn pour<D: IntoDiagnostic>(&mut self, diagnostic: D) {
        let diagnostic = diagnostic.into_diagnostic();
        eprintln!("{}", diagnostic);
    }
}

impl InstantStdioDiagnosticStream {
    pub fn new() -> Self {
        Self
    }
}

impl Default for InstantStdioDiagnosticStream {
    fn default() -> Self {
        Self::new()
    }
}

pub struct StockDiagnosticStream {
    pub diagnostics: Vec<Diagnostic>,
}

impl DiagnosticStream for StockDiagnosticStream {
    fn pour<D: IntoDiagnostic>(&mut self, error: D) {
        let diagnostic = error.into_diagnostic();
        self.diagnostics.push(diagnostic);
    }
}

impl StockDiagnosticStream {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    #[cfg(debug_assertions)]
    pub fn success(&self) -> bool {
        self.diagnostics.is_empty()
    }

    pub fn into_vec(self) -> Vec<Diagnostic> {
        self.diagnostics
    }
}

impl Default for StockDiagnosticStream {
    fn default() -> Self {
        Self::new()
    }
}

pub struct IgnoreDiagnosticStream;

impl DiagnosticStream for IgnoreDiagnosticStream {
    fn pour<D: IntoDiagnostic>(&mut self, _: D) {}
}

impl IgnoreDiagnosticStream {
    pub fn new() -> Self {
        Self
    }
}

impl Default for IgnoreDiagnosticStream {
    fn default() -> Self {
        Self::new()
    }
}
