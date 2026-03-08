use std::{fmt, io};

use crate::compiler::span::Span;

pub trait Tracer: Sized {
    type Guard;
    fn enter(name: &'static str);
    fn leave(name: &'static str);
    fn trace(name: &'static str) -> Self::Guard;
}

pub struct NopeTracer;

impl Tracer for NopeTracer {
    type Guard = ();
    fn enter(_: &'static str) {}
    fn leave(_: &'static str) {}
    fn trace(name: &'static str) -> Self::Guard {}
}

pub struct LogTracer;
impl Tracer for LogTracer {
    type Guard = TraceGuard;
    fn enter(name: &'static str) {
        print!("<{}>", name);
    }

    fn leave(name: &'static str) {
        print!("</{}>", name);
    }

    fn trace(name: &'static str) -> TraceGuard {
        Self::enter(name);
        TraceGuard { name }
    }
}

pub struct TraceGuard {
    name: &'static str,
}

impl Drop for TraceGuard {
    fn drop(&mut self) {
        print!("</{}>", self.name);
    }
}
