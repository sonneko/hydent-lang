use crate::diagnostic::CompilerDiagnostic;

pub struct ErrorPool {
    errors: Vec<Box<dyn CompilerDiagnostic>>,
}

impl ErrorPool {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn add<T>(&mut self, error: T)
    where
        T: CompilerDiagnostic,
    {
        self.errors.push(Box::new(error));
    }

    fn show(self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for error in self.errors {
            // TODO
        }
        Ok(())
    }
}
