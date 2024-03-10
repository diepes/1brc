use criterion::{criterion_group, criterion_main, Criterion};

use brc::main_brc;

fn bench_1_billion_rows(c: &mut Criterion) {
    c.bench_function("bench_1brc", |b| {
        b.iter(|| {
            //std::hint::black_box(for _i in 1..=100 {
                main_brc("./measurements_1million.txt",false);
            //});
        });
    });
}

// criterion_group!(benches, bench_1_billion_rows,);
criterion_group!{
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().significance_level(0.1).sample_size(100);
    targets = bench_1_billion_rows
}
criterion_main!(benches);
