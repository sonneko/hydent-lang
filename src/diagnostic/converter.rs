use crate::diagnostic::Diagnostic;

pub trait IntoDiagnostic {
    type Reference;
    fn into_diagnostic(self, reference: &Self::Reference) -> Diagnostic;
}
