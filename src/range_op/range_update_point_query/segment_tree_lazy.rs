struct SegTree {
    t: Vec<u32>,
    d: Vec<u32>,
}

impl SegTree {
    fn new(n: usize) -> Self {
        Self { t: vec![0; 2 * n],
               d: vec![u32::MAX; n] }
    }

    fn assign(&mut self, l0: usize, r0: usize, v: u32) {
        let (l0, r0) = (l0 + self.d.len(), r0 + self.d.len());
        self._push(l0);
        self._push(r0);
        let (mut l, mut r) = (l0, r0);
        while l <= r {
            if l % 2 == 1 {
                self._apply(l, v);
                l += 1;
            }

            if r % 2 == 0 {
                self._apply(r, v);
                r -= 1;
            }

            (l, r) = (l / 2, r / 2);
        }

        self._pull(l0);
        self._pull(r0);
    }

    fn get(&mut self, mut i: usize) -> u32 {
        i += self.d.len();
        self._push(i);

        self.t[i]
    }

    fn _push(&mut self, i: usize) {
        for s in (1..=self.d.len().ilog2()).rev() {
            let i = i >> s;
            if self.d[i] != u32::MAX {
                self._apply(2 * i, self.d[i]);
                self._apply(2 * i + 1, self.d[i]);
                self.d[i] = u32::MAX;
            }
        }
    }

    fn _apply(&mut self, i: usize, v: u32) {
        self.t[i] = v;
        if i < self.d.len() {
            self.d[i] = v;
        }
    }

    fn _pull(&mut self, mut i: usize) {
        while i > 1 {
            i /= 2;
            self.t[i] = self.d[i];
        }
    }
}
