use crate::{
    diagnostic::converter::IntoDiagnostic, parser::errors::ParseErr, tokenizer::errors::TokenizeErr,
};

impl IntoDiagnostic for ParseErr {
    fn into_diagnostic(self) -> crate::diagnostic::Diagnostic {
        todo!()
    }
}

impl IntoDiagnostic for TokenizeErr {
    fn into_diagnostic(self) -> crate::diagnostic::Diagnostic {
        todo!()
    }
}
