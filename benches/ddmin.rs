use algorithms::ddmin::ddmin_set::ddmin;
use criterion::{Criterion, criterion_group, criterion_main};
use std::{collections::BTreeSet, hint::black_box};

fn mytest(inp: &BTreeSet<&i32>) -> bool {
    !(inp.contains(&536) && inp.contains(&48143) && inp.contains(&279958))
}

fn criterion_benchmark(c: &mut Criterion) {
    let x = (0..300_000).collect::<Vec<i32>>();
    let y = x.iter().collect::<BTreeSet<&i32>>();

    c.bench_function("ddmin", |b| {
        b.iter(|| ddmin(mytest, black_box(&y), true));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
