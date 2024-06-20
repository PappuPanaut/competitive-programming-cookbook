/// Uses 1-based indexing
struct FenwickTree {
    t: Vec<i64>,
}

impl FenwickTree {
    fn with_len(n: usize) -> Self {
        Self { t: vec![0; n + 1] }
    }

    fn prefix_sum(&self, mut i: usize) -> i64 {
        let mut prefix_sum = 0;

        while i > 0 {
            prefix_sum += self.t[i];

            i -= lsb(i);
        }

        prefix_sum
    }

    fn range_sum(&self, range: std::ops::RangeInclusive<usize>) -> i64 {
        self.prefix_sum(*range.end()) - self.prefix_sum(*range.start() - 1)
    }

    fn get(&self, i: usize) -> i64 {
        self.range_sum(i..=i)
    }

    fn increment(&mut self, mut i: usize, dt: i64) {
        while i < self.t.len() {
            self.t[i] += dt;

            i += lsb(i);
        }
    }

    fn set(&mut self, i: usize, x: i64) {
        self.increment(i, x - self.get(i));
    }
}

fn lsb(i: usize) -> usize {
    let i = i as isize;

    (i & -i) as _
}
