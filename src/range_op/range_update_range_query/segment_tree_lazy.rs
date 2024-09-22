struct SegTree {
    t: Vec<u32>,
    d: Vec<u32>,
}

impl SegTree {
    fn new(n: usize) -> Self {
        Self { t: vec![0; 2 * n],
               d: vec![0; n] }
    }

    fn add(&mut self, l0: usize, r0: usize, v: u32) {
        let (l0, r0) = (l0 + self.d.len(), r0 + self.d.len());

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

    fn rng_min(&mut self, l: usize, r: usize) -> u32 {
        let mut rng_min = u32::MAX;

        let (mut l, mut r) = (l + self.d.len(), r + self.d.len());
        self._push(l);
        self._push(r);
        while l <= r {
            if l % 2 == 1 {
                rng_min = rng_min.min(self.t[l]);
                l += 1;
            }

            if r % 2 == 0 {
                rng_min = self.t[r].min(rng_min);
                r -= 1;
            }

            (l, r) = (l / 2, r / 2);
        }

        rng_min
    }

    fn _push(&mut self, i: usize) {
        for s in (1..=self.d.len().ilog2()).rev().map(|s| i >> s) {
            if self.d[s] != 0 {
                self._apply(2 * s, self.d[s]);
                self._apply(2 * s + 1, self.d[s]);
                self.d[s] = 0;
            }
        }
    }

    fn _apply(&mut self, i: usize, v: u32) {
        self.t[i] += v;
        self.d[i] += if i < self.d.len() { v } else { 0 }
    }

    fn _pull(&mut self, mut i: usize) {
        while i > 1 {
            i /= 2;
            self.t[i] = self.t[2 * i].min(self.t[2 * i + 1]) + self.d[i];
        }
    }
}
