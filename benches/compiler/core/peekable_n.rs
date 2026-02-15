use criterion::{criterion_group, Criterion};
use hydent_lang_compiler::utility::peekable_n::PeekableN;

fn bench_peekable_n_creation(c: &mut Criterion) {
    c.bench_function("peekable_n_new", |b| {
        b.iter(|| {
            let data = [1, 2, 3, 4, 5];
            let _ = PeekableN::<_, _, 4>::new(data.into_iter());
        })
    });
}

fn bench_peekable_n_next(c: &mut Criterion) {
    c.bench_function("peekable_n_next", |b| {
        b.iter(|| {
            let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            let mut iter = PeekableN::<_, _, 4>::new(data.into_iter());
            while iter.next().is_some() {}
        })
    });
}

criterion_group!(
    peekable_n_benches,
    bench_peekable_n_creation,
    bench_peekable_n_next
);
