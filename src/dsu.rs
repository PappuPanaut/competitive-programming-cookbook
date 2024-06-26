struct Dsu {
    pa: Vec<u32>,
}

impl Dsu {
    fn with_len(n: usize) -> Self {
        Self {
            pa: (0..n as _).collect(),
        }
    }

    fn rep(&mut self, u: usize) -> usize {
        if u == self.pa[u] as _ {
            return u as _;
        }

        self.pa[u] = self.rep(self.pa[u] as _) as _;

        self.pa[u] as _
    }

    fn merge(&mut self, u: usize, v: usize) {
        let (ru, rv) = (self.rep(u), self.rep(v));
        self.pa[ru] = rv as _;
    }
}
