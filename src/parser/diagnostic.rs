use crate::{
    diagnostic::{converter::IntoDiagnostic, Diagnostic},
    parser::errors::ParseErr,
    tokenizer::errors::TokenizeErr,
};

impl IntoDiagnostic for ParseErr {
    type Reference = crate::parser::base_parser::Enviroment;
    fn into_diagnostic(self, reference: &Self::Reference) -> crate::diagnostic::Diagnostic {
        println!("{:?}", self);
        Diagnostic::default()
    }
}

impl IntoDiagnostic for TokenizeErr {
    type Reference = ();
    fn into_diagnostic(self, _: &()) -> crate::diagnostic::Diagnostic {
        eprintln!("tokenize error: {:?}", self);
        Diagnostic::default()
    }
}
