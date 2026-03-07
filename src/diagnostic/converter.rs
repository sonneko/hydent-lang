use crate::diagnostic::Diagnostic;

pub trait IntoDiagnostic {
    fn into_diagnostic(self) -> Diagnostic;
}
