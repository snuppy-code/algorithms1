pub fn mergesort<T: Copy + PartialOrd>(inp: Vec<T>) -> Vec<T> {
    let l = inp.len();
    let mut out = Vec::new();
    if l > 1 {
        let (a, b) = split_into_two(inp);
        let (a, b) = (mergesort(a), mergesort(b));
        let (mut a_min, mut b_min): (usize, usize) = (0, 0);

        while a_min < a.len() && b_min < b.len() {
            if a[a_min] > b[b_min] {
                out.push(b[b_min]);
                b_min += 1;
            } else {
                out.push(a[a_min]);
                a_min += 1;
            }
        }
        while a_min < a.len() {
            out.push(a[a_min]);
            a_min += 1;
        }
        while b_min < b.len() {
            out.push(b[b_min]);
            b_min += 1;
        }
        return out;
    } else {
        return inp;
    }
}
fn split_into_two<T>(inp: Vec<T>) -> (Vec<T>, Vec<T>) {
    let l = inp.len();
    let mut it = inp.into_iter();
    let it = it.by_ref();

    return (it.take(l / 2).collect(), it.collect());
}
