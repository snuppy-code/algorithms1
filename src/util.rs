use std::collections::BTreeSet;

/// e.g. balanced_partition({1, 2, 3, 4, 5},n) for n = 2,3,4:
/// [{1, 2, 3}, {4, 5}]
/// [{1, 2}, {3, 4}, {5}]
/// [{1, 2}, {3}, {4}, {5}]
pub fn balanced_partition<T: Ord + Copy>(set: &BTreeSet<T>, n: usize) -> Vec<BTreeSet<T>> {
    let l = set.len();
    let excess = set.len() % n;

    let mut inp = set.iter().copied();
    let inp = inp.by_ref();

    let mut partition = Vec::with_capacity(n);

    partition.extend((0..excess).map(|_| inp.take(l / n + 1).collect()));
    partition.extend((excess..n).map(|_| inp.take(l / n).collect()));

    partition
}
