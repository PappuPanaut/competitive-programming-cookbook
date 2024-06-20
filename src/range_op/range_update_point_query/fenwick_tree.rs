/// Uses 1-based indexing
struct FenwickTree {
    t: Vec<i64>,
}

impl FenwickTree {
    fn with_len(n: usize) -> Self {
        Self { t: vec![0; n + 1] }
    }

    fn increment(&mut self, i: usize, dt: i64) {
        self.increment_range(i..=i, dt);
    }

    fn set(&mut self, i: usize, x: i64) {
        self.increment(i, x - self.get(i));
    }

    fn increment_range(&mut self, range: std::ops::RangeInclusive<usize>, dt: i64) {
        self._increment(*range.start(), dt);
        self._increment(1 + *range.end(), -dt);
    }

    fn get(&self, mut i: usize) -> i64 {
        let mut res = 0;

        while i > 0 {
            res += self.t[i];

            i -= lsb(i);
        }

        res
    }

    fn _increment(&mut self, mut i: usize, dt: i64) {
        while i < self.t.len() {
            self.t[i] += dt;

            i += lsb(i);
        }
    }
}

fn lsb(i: usize) -> usize {
    let i = i as isize;

    (i & -i) as _
}
