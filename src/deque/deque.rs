struct Deque {
    start: usize,
    len: usize,
    dynArr: Vec<Option<i32>>,
}

impl Deque {
    fn new() -> Self {
        return Deque {
            start: 0,
            len: 0,
            dynArr: Vec::new(),
        };
    }
    fn push_front(&mut self, v: i32) {
        if self.len < (self.dynArr.len() + 1) {
            if self.dynArr.len() == 0 {
                self.dynArr.push(None);
            } else {
                self.dynArr.len()
            }
        }
        self.dynArr.push(v);
    }
    fn push_end(&mut self, v: i32) {

}
    fn get(&mut self, i: usize) {

}
}
