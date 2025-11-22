use std::fmt::Debug;

pub struct ParseErr<'a> {
    message: &'a str,
    err_kind: ParseErrKind,
}

pub enum ParseErrKind {
    // INFO: add error kind here
}

impl<'a> Debug for ParseErr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}