/// Uses 1-based indexing
struct FenwickTree {
    t: Vec<i64>,
}

impl FenwickTree {
    fn new(n: usize) -> Self { Self { t: vec![0; n + 1] } }

    fn add_rng(&mut self, rng: std::ops::RangeInclusive<usize>, dt: i64) {
        self._add(*rng.start(), dt);
        self._add(1 + *rng.end(), -dt);
    }

    fn get(&self, mut i: usize) -> i64 {
        let mut res = 0;

        while i > 0 {
            res += self.t[i];
            i -= (i as isize & -(i as isize)) as usize;
        }

        res
    }

    fn _add(&mut self, mut i: usize, dt: i64) {
        while i < self.t.len() {
            self.t[i] += dt;
            i += (i as isize & -(i as isize)) as usize;
        }
    }
}
