struct SparseTable {
    t: Vec<Vec<u32>>,
}

impl SparseTable {
    fn new(s: &[u32]) -> Self {
        let mut t = vec![vec![0; s.len()]; 1 + s.len().ilog2() as usize];
        t[0] = s.to_vec();

        for b in 1..t.len() {
            for i in 0..=s.len() - (1 << b) {
                t[b][i] = t[b - 1][i].min(t[b - 1][i + (1 << (b - 1))]);
            }
        }

        Self { t }
    }

    fn min_rng(&self, l: usize, r: usize) -> u32 {
        let b = (r - l + 1).ilog2() as usize;

        self.t[b][l].min(self.t[b][r + 1 - (1 << b)])
    }
}
