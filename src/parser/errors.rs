pub struct ParseErr;

impl ParseErr {
    fn new() -> Self {
        Self
    }
}


impl std::fmt::Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}