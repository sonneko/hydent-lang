#[macro_use]
extern crate criterion;

mod compiler;

mod arena_allocater {
    use criterion::Criterion;

    fn allocate_box(c: &mut Criterion) {
        c.bench_function("allocate_box_once", |b| b.iter(|| {}));
    }

    criterion_group!(arena_allocater, allocate_box);
}

criterion_main!(arena_allocater::arena_allocater);
