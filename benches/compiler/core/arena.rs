use criterion::{criterion_group, Criterion};
use std::hint::black_box;

use hydent_lang_compiler::compiler::arena::Arena;

fn bench_arena_creation(c: &mut Criterion) {
    c.bench_function("arena_creation", |b| {
        b.iter(|| {
            black_box(Arena::new());
        })
    });
}

fn bench_arena_alloc_single(c: &mut Criterion) {
    c.bench_function("arena_alloc_single_1000", |b| {
        b.iter(|| {
            let arena = Arena::new();
            for _ in 0..1000 {
                black_box(arena.alloc(black_box(42)));
            }
        })
    });
}

fn bench_arena_alloc_iter(c: &mut Criterion) {
    c.bench_function("arena_alloc_iter_1000", |b| {
        b.iter(|| {
            let arena = Arena::new();
            let iter = 0..1000;
            black_box(arena.alloc_iter(iter));
        })
    });
}

criterion_group!(
    arena_benches,
    bench_arena_creation,
    bench_arena_alloc_single,
    bench_arena_alloc_iter
);
