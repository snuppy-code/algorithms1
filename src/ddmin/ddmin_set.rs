use std::collections::BTreeSet;

/// It seems like without chunking based on extra info about the inputs,
/// the best we can do is some kind of balanced partition on `inp`.
/// "balanced" as in the length of every chunk is +-1 of each other or equal.
fn balanced_partition<'a, T: Ord>(inp: &BTreeSet<&'a T>, n: usize) -> Vec<BTreeSet<&'a T>> {
    assert!(n >= 2, "won't split into n>2 chunks !");
    assert!(n <= inp.len(), "won't split into n>inp.len() chunks !");

    let each = inp.len() / n;
    let excess = inp.len() % n;
    let mut inp = inp.iter();

    let mut excess_parts = (0..excess).map(|_| inp.take(each + 1));
    let mut small_parts = (excess..n).map(|_| inp.take(each));

    excess_parts.chain(small_parts).collect()
}

/// Internal implementation of outer `ddmin` function.
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

/// BTreeSet is ordered, which means (if `test` is well behaved) this function
/// is deterministic, and not just as an implementation detail.
/// This Delta Debugging implementation intentionally does not attempt to do
/// any smart chunking or keeping related objects together.
///
/// `test` should return false if its inputs induced the relevant failure.
///
/// Delta Debugging only works if the input fails, so this should be tested
/// on first run, but `test` may be terribly slow to run, in which case it
/// is preferable to let the user pinky promise it fails, and skip this first check.
pub fn ddmin<'a, T: Ord>(
    mut test: impl FnMut(&BTreeSet<&T>) -> bool,
    inp: &BTreeSet<&'a T>,
    assume_input_fails: bool,
) -> BTreeSet<&'a T> {
    if !assume_input_fails {
        assert!(!test(&inp), "input must fail !");
    }
    _ddmin(test, inp, 2)
}
