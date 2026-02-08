pub mod diagnotice_patterns;
pub mod error_pool;

use crate::compiler::span::Span;

/// Represents a highlighted region in the source code for diagnostic purposes.
///
/// This struct is used to point out specific parts of the code that are
/// relevant to a diagnostic message, optionally with a label.
#[derive(Debug)]
pub struct Highlight<'ms> {
    pub span: Span,
    pub label: Option<&'ms str>,
    pub is_primary: bool,
}

/// Represents a suggestion for how to fix a diagnostic issue.
///
/// This includes a message explaining the suggestion, and optionally
/// a replacement span and text to apply the suggestion programmatically.
#[derive(Debug)]
pub struct Suggestion<'ms> {
    pub message: &'ms str,
    pub replacement_span: Option<Span>,
    pub replacement_text: Option<&'ms str>,
}

/// Represents the severity level of a diagnostic message.
pub enum DiagnosticLevel {
    /// An error indicates a critical issue that prevents compilation.
    Error,
    /// A warning indicates a potential issue that might not prevent compilation but should be addressed.
    Warning,
    /// A note provides additional information or context for a diagnostic.
    Note,
}

/// A trait for defining custom diagnostic patterns in the compiler.
///
/// Implementors of this trait provide details about a specific diagnostic
/// issue, including its error code, severity level, primary message and span,
/// additional highlights, notes, and suggestions for resolution.
pub trait CompilerDiagnostic: Send + Sync + 'static {
    /// Returns the unique error code for this diagnostic.
    ///
    /// This code is typically used for referencing documentation or for filtering diagnostics.
    fn error_code(&self) -> u16;

    /// Returns the severity level of this diagnostic.
    fn level(&self) -> DiagnosticLevel;

    /// Returns the primary message associated with this diagnostic.
    ///
    /// This message should concisely describe the issue.
    fn primary_message(&self) -> &str;

    /// Returns the primary span in the source code where this diagnostic occurs.
    ///
    /// This is the main location pointed to by the diagnostic.
    fn primary_span(&self) -> Span;

    /// Returns a list of additional highlights in the source code related to this diagnostic.
    ///
    /// These highlights can provide further context to the user.
    fn highlights(&self) -> Vec<Highlight<'_>>;

    /// Returns a list of supplementary notes or explanations for this diagnostic.
    fn notes(&self) -> Vec<&str>;

    /// Returns a list of suggestions for how to resolve this diagnostic.
    ///
    /// These can include code replacements or general advice.
    fn suggestions(&self) -> Vec<Suggestion<'_>>;

    /// Returns an optional URL pointing to more detailed documentation for this diagnostic.
    ///
    /// The default implementation generates a URL based on the error code.
    fn documentation_url(&self) -> Option<String> {
        Some(format!(
            "https://doc.****.com/error_codes/E{:04}.html",
            self.error_code()
        ))
    }
}
