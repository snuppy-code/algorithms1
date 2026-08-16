use std::collections::BTreeSet;

fn balanced_partition<'a, T: Ord>(inp: &BTreeSet<&'a T>, n: usize) -> Vec<BTreeSet<&'a T>> {
    assert!(n >= 2, "won't split into n>2 chunks !");
    assert!(n <= inp.len(), "won't split into n>inp.len() chunks !");

    let each = inp.len() / n;
    let excess = inp.len() % n;
    let mut inp = inp.iter();

    (0..n)
        .map(|i| {
            let add = if i < excess { 1 } else { 0 };
            inp.by_ref().take(each + add).copied().collect()
        })
        .collect()
}

fn _ddmin<'a, T>(
    mut test: impl FnMut(&BTreeSet<&T>) -> bool,
    inp: &BTreeSet<&'a T>,
    n: usize,
) -> BTreeSet<&'a T>
where
    T: Ord,
{
    assert!(n >= 2, "n must be at least 2 !");

    if inp.len() == 1 {
        return inp.clone(); // 1-minimality achieved!
    }

    let chunks = balanced_partition(&inp, n);

    // does any chunk fail on its own?
    for c in &chunks {
        if !test(&c) {
            return _ddmin(test, c, 2);
        }
    }
    // does inp still fail after we remove any chunk?
    for c in &chunks {
        let compl = inp.difference(&c).copied().collect();
        if !test(&compl) {
            // proceed with all chunks but the one whose complement failed
            return _ddmin(test, &compl, std::cmp::max(2, n - 1));
        }
    }
    if inp.len() == n {
        return inp.clone(); // 1-minimality achieved!
    }

    // try cutting finer chunks !
    return _ddmin(test, inp, std::cmp::min(n * 2, inp.len()));
}

pub fn ddmin<'a, T: Ord>(
    mut test: impl FnMut(&BTreeSet<&T>) -> bool,
    inp: &BTreeSet<&'a T>,
) -> BTreeSet<&'a T> {
    assert!(!test(&inp), "input must fail !");
    _ddmin(test, inp, 2)
}
