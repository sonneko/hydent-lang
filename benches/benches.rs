#[macro_use]
extern crate criterion;

mod compiler;

criterion_main!(
    compiler::core::peekable_n::peekable_n_benches,
    compiler::core::arena::arena_benches
);
