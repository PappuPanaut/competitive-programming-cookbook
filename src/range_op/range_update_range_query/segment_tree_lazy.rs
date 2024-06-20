struct SegmentTree {
    t: Vec<u32>,
    d: Vec<u32>,
}

impl SegmentTree {
    fn with_len(n: usize) -> Self {
        Self {
            t: vec![0; 2 * n],
            d: vec![0; n],
        }
    }

    fn range_min(&mut self, l: usize, r: usize) -> u32 {
        let mut range_min = u32::MAX;
        let (mut l, mut r) = (l + self.d.len(), r + self.d.len());
        self.push(l);
        self.push(r);

        while l <= r {
            if l % 2 == 1 {
                range_min = range_min.min(self.t[l]);
                l += 1;
            }

            if r % 2 == 0 {
                range_min = self.t[r].min(range_min);
                r -= 1;
            }

            (l, r) = (l / 2, r / 2);
        }

        range_min
    }

    fn push(&mut self, i: usize) {
        for s in (1..=self.d.len().ilog2()).rev() {
            let i = i >> s;

            if self.d[i] == 0 {
                continue;
            }

            self.apply(2 * i, self.d[i]);
            self.apply(2 * i + 1, self.d[i]);
            self.d[i] = 0;
        }
    }

    fn apply(&mut self, i: usize, v: u32) {
        self.t[i] += v;

        if i < self.d.len() {
            self.d[i] += v;
        }
    }

    fn increment(&mut self, l0: usize, r0: usize, v: u32) {
        let (l0, r0) = (l0 + self.d.len(), r0 + self.d.len());

        let (mut l, mut r) = (l0, r0);
        while l <= r {
            if l % 2 == 1 {
                self.apply(l, v);
                l += 1;
            }

            if r % 2 == 0 {
                self.apply(r, v);
                r -= 1;
            }

            (l, r) = (l / 2, r / 2);
        }

        self.pull(l0);
        self.pull(r0);
    }

    fn pull(&mut self, mut i: usize) {
        while i > 1 {
            i /= 2;
            self.t[i] = self.t[2 * i].min(self.t[2 * i + 1]) + self.d[i];
        }
    }
}
