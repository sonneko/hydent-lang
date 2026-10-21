use crate::{
    diagnostic::{converter::IntoDiagnostic, Diagnostic, DiagnosticLevel, Highlight},
    parser::{base_parser::Enviroment, errors::ParseErr},
    tokenizer::errors::TokenizeErr,
};

impl IntoDiagnostic for ParseErr {
    type Reference = Enviroment;

    fn into_diagnostic(self, reference: &Self::Reference) -> Diagnostic {
        let found = self.found();
        let message = if found.current == crate::tokenizer::tokens::Token::EndOfFile {
            "unexpected end of file"
        } else {
            match self.expected() {
                [] => "unexpected token",
                [_] => "unexpected token; expected a specific token",
                _ => "unexpected token; expected one of several tokens",
            }
        };

        Diagnostic {
            id: 1001,
            message,
            primary: reference.span,
            level: DiagnosticLevel::Error,
            highlights: vec![Highlight {
                span: reference.span,
                label: Some("unexpected token"),
                is_primary: true,
            }],
            suggestions: Vec::new(),
        }
    }
}

impl IntoDiagnostic for TokenizeErr {
    type Reference = ();

    fn into_diagnostic(self, _: &()) -> Diagnostic {
        use crate::tokenizer::errors::TokenizeErrKind;

        let message = match self.kind() {
            TokenizeErrKind::StringLiteralNotClosed => "unterminated string literal",
            TokenizeErrKind::CharLiteralNotClosed => "unterminated character literal",
            TokenizeErrKind::InvalidCharLiteral => "invalid character literal",
            TokenizeErrKind::InvalidIntegerLiteral => "invalid integer literal",
            TokenizeErrKind::InvalidFloatLiteral => "invalid float literal",
            TokenizeErrKind::UnknownToken => "unknown token",
            TokenizeErrKind::BlockCommentNotClosed => "unterminated block comment",
        };
        let span = self.span();

        Diagnostic {
            id: 1002,
            message,
            primary: span,
            level: DiagnosticLevel::Error,
            highlights: vec![Highlight {
                span,
                label: Some("invalid token"),
                is_primary: true,
            }],
            suggestions: Vec::new(),
        }
    }
}
