use std::collections::BTreeSet;

/// e.g. balanced_partition({1, 2, 3, 4, 5},n) for n = 2,3,4:
/// [{1, 2, 3}, {4, 5}]
/// [{1, 2}, {3, 4}, {5}]
/// [{1, 2}, {3}, {4}, {5}]
pub fn balanced_partition_btset<T: Ord + Copy>(set: &BTreeSet<T>, n: usize) -> Vec<BTreeSet<T>> {
    let l = set.len();
    let rem = set.len() % n;

    let mut inp = set.iter().copied();
    let inp = inp.by_ref();

    let mut partition = Vec::with_capacity(n);

    partition.extend((0..rem).map(|_| inp.take(l / n + 1).collect()));
    partition.extend((rem..n).map(|_| inp.take(l / n).collect()));

    partition
}

pub fn balanced_partition_vec<T: Ord + Copy>(set: &Vec<T>, n: usize) -> Vec<Vec<T>> {
    let l = set.len();
    let rem = set.len() % n;

    let mut inp = set.iter().copied();
    let inp = inp.by_ref();

    let mut partition = Vec::with_capacity(n);

    partition.extend((0..rem).map(|_| inp.take(l / n + 1).collect()));
    partition.extend((rem..n).map(|_| inp.take(l / n).collect()));

    partition
}
