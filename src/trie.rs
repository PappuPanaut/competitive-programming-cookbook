struct Trie {
    t: Vec<[usize; 2]>,
}

impl Trie {
    fn new() -> Self { Self { t: vec![[0, 0]] } }

    fn add(&mut self, x: usize) {
        let mut v = 0;
        for xb in (0..41).rev().map(|b| (x >> b) & 1) {
            if self.t[v][xb] == 0 {
                self.t[v][xb] = self.t.len();
                self.t.push([0, 0]);
            }

            v = self.t[v][xb];
        }
    }

    fn max_xor(&self, x: usize) -> u64 {
        let mut res = 0;

        let mut v = 0;
        for xb in (0..41).rev().map(|b| (x >> b) & 1) {
            res *= 2;
            if self.t[v][1 ^ xb] == 0 {
                v = self.t[v][xb]
            } else {
                res += 1;
                v = self.t[v][1 ^ xb];
            }
        }

        res
    }
}
