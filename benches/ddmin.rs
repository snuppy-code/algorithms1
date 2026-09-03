use algorithms::ddmin::ddmin_set::ddmin;
use criterion::{Criterion, criterion_group, criterion_main};
use rand::prelude::*;
use std::{collections::BTreeSet, hint::black_box};

fn criterion_benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("ddmin");

    let set_size: usize = 500;
    let troublemakers: usize = 6;

    let set = (0..set_size).collect::<BTreeSet<usize>>();

    let nums: BTreeSet<usize> = {
        let mut rng = rand::rng();
        let mut nums = BTreeSet::new();
        while nums.len() < troublemakers {
            nums.insert(rng.random_range(0..set_size));
        }
        nums
    };
    let test = |v: &BTreeSet<usize>| v.is_disjoint(&nums);

    g.bench_function("w/ sets", |b| {
        b.iter(|| ddmin(black_box(test), black_box(&set), true));
    });
    g.bench_function("w/ vecs", |b| {
        b.iter(|| ddmin(black_box(test), black_box(&set), true));
    });

    g.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
