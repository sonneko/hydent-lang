use std::io::Read;

use crate::compiler::runtime::Query;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FileOpenErr {
    NotFound,
    PermissionDenied,
    LockedByAnotherProcess,
    FileTooLarge,
    TooManyOpenFiles,
}

pub struct ReadFileQuery;
impl Query for ReadFileQuery {
    type From = String;
    type To = Result<String, FileOpenErr>;
    fn run<E: crate::compiler::runtime::Engine>(engine: &E, src: Self::From) -> Self::To {
        // check if file has edited
        let mut file = std::fs::File::open(&src);

        // read file

        // return file contents as string
        todo!()
    }
}
