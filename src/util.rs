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

// pub fn balanced_partition<T: Ord + Copy>(inp: &BTreeSet<T>, n: usize) -> Vec<BTreeSet<T>> {
//     assert!(n >= 2, "won't split into n>2 chunks !");
//     assert!(n <= inp.len(), "won't split into n>inp.len() chunks !");

//     let each = inp.len() / n;
//     let excess = inp.len() % n;
//     let mut inp = inp.iter();

//     (0..n)
//         .map(|i| {
//             let add = if i < excess { 1 } else { 0 };
//             inp.by_ref().take(each + add).copied().collect()
//         })
//         .collect()
// }

// pub fn balanced_partition<T: Ord + Copy>(inp: &BTreeSet<T>, n: usize) -> Vec<BTreeSet<T>> {
//     assert!(n >= 2, "won't split into n>2 chunks !");
//     assert!(n <= inp.len(), "won't split into n>inp.len() chunks !");

//     let each = inp.len() / n;
//     let excess = inp.len() % n;
//     let mut inp = inp.iter();
//     let a = &mut inp;

//     let mut excess_parts = (0..excess)
//         .map(|_| a.take(each + 1).copied().collect::<BTreeSet<T>>())
//         .collect::<Vec<BTreeSet<T>>>();
//     let mut small_parts = (excess..n)
//         .map(|_| a.take(each).copied().collect::<BTreeSet<T>>())
//         .collect::<Vec<BTreeSet<T>>>();

//     excess_parts.append(&mut small_parts);
//     let chunks = excess_parts;

//     chunks
// }

// pub fn balanced_partition<T: Ord + Copy>(inp: &BTreeSet<T>, n: usize) -> Vec<BTreeSet<T>> {
//     assert!(n >= 2, "won't split into n>2 chunks !");
//     assert!(n <= inp.len(), "won't split into n>inp.len() chunks !");

//     let mut inp = inp.iter();

//     (1..=n)
//         .rev()
//         .map(|rem_chunks| {
//             let rem_len = inp.len();
//             inp.by_ref()
//                 .take(rem_len.div_ceil(rem_chunks))
//                 .copied()
//                 .collect()
//         })
//         .collect()
// }

// pub fn balanced_partition<T: Ord + Copy>(inp: &BTreeSet<T>, n: usize) -> Vec<BTreeSet<T>> {
//     assert!(n >= 2, "won't split into n>2 chunks !");
//     assert!(n <= inp.len(), "won't split into n>inp.len() chunks !");

//     let each = inp.len() / n;
//     let excess = inp.len() % n;
//     let mut inp = inp.iter();

//     let excess_parts = (0..excess)
//         .map(|_| inp.by_ref().take(each + 1).copied().collect())
//         .collect::<Vec<BTreeSet<T>>>();

//     excess_parts
//         .into_iter()
//         .chain((excess..n).map(|_| inp.by_ref().take(each).copied().collect()))
//         .collect()
// }
