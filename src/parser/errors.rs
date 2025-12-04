use std::fmt::Debug;

use crate::tokenizer::Token;

pub enum ParseErr {
    UnexpectedToken {
        message: &'static str,
        expected: Vec<Token>,
        found: Option<Token>,
    }
    // INFO: add error kind here
}

impl<'a> ParseErr<'a> {
    pub fn unexpected(expected: Vec<Token>, found: Option<Token>) -> Option<ParseErr<'a>> {
        Some(&Self::UnexpectedToken {
            message: "Unexpected Token",
            expected,
            found,
        })
    }
}

impl<'a> Debug for ParseErr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}