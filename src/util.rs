use std::collections::BTreeSet;

pub fn balanced_partition<'a, T: Ord + Copy>(inp: &BTreeSet<T>, n: usize) -> Vec<BTreeSet<T>> {
    let each = inp.len() / n;
    let excess = inp.len() % n;
    let mut inp = inp.iter().copied();
    let inp = inp.by_ref();

    let mut partition = Vec::with_capacity(n);

    partition.extend((0..excess).map(|_| inp.take(each + 1).collect()));
    partition.extend((excess..n).map(|_| inp.take(each).collect()));

    partition
}
