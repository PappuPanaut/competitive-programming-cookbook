struct SegmentTree {
    t: Vec<u32>,
    d: Vec<u32>,
    n: usize,
    h: usize,
}

impl SegmentTree {
    fn with_len(n: usize) -> Self {
        Self {
            t: vec![0; 2 * n],
            d: vec![u32::MAX; n],
            n,
            h: n.ilog2() as _,
        }
    }

    fn get(&mut self, mut i: usize) -> u32 {
        i += self.n;
        self.push(i);

        self.t[i]
    }

    fn push(&mut self, i: usize) {
        for s in (1..=self.h).rev() {
            let i = i >> s;

            if self.d[i] == u32::MAX {
                continue;
            }

            self.apply(2 * i, self.d[i]);
            self.apply(2 * i + 1, self.d[i]);
            self.d[i] = u32::MAX;
        }
    }

    fn apply(&mut self, i: usize, v: u32) {
        self.t[i] = v;

        if i < self.n {
            self.d[i] = v;
        }
    }

    fn assign(&mut self, l0: usize, r0: usize, v: u32) {
        let (l0, r0) = (l0 + self.n, r0 + self.n);
        self.push(l0);
        self.push(r0);

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

        self.build(l0);
        self.build(r0);
    }

    fn build(&mut self, mut i: usize) {
        while i > 1 {
            i /= 2;
            self.t[i] = self.d[i];
        }
    }
}
