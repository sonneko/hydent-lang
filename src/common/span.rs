use core::hash::Hash;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Span {
    begin: usize,
    end: usize,
}

impl Span {
    pub fn new(begin: usize, end: usize) -> Self {
        Self { begin, end }
    }

    pub fn with_ref(&self, all: &str) -> SpanWithRef {
        SpanWithRef {
            span: self,
            reference: all[self.begin..self.end],
        }
    }
}

pub struct SpanWithRef<'a> {
    span: Span,
    reference: &'a str,
}

impl<'a> Hash for SpanWithRef<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.reference.hash(state);
    }
}
