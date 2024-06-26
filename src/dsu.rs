struct Dsu {
    pa: Vec<u32>,
    sz: Vec<u32>,
    num_cc: u32,
    cc_mx_sz: u32,
}

impl Dsu {
    fn with_len(n: usize) -> Self {
        Self {
            pa: (0..n).map(|i| i as u32).collect(),
            sz: vec![1; n],
            num_cc: n as _,
            cc_mx_sz: 1,
        }
    }

    fn rep(&mut self, u: usize) -> usize {
        if u == self.pa[u] as _ {
            return u;
        }

        self.pa[u] = self.rep(self.pa[u] as _) as _;

        self.pa[u] as _
    }

    fn merge(&mut self, u: usize, v: usize) {
        let (ru, rv) = (self.rep(u), self.rep(v));

        if ru == rv {
            return;
        }

        self.pa[ru] = rv as _;
        self.sz[rv] += self.sz[ru];
        self.cc_mx_sz = self.cc_mx_sz.max(self.sz[rv]);
        self.num_cc -= 1;
    }
}
