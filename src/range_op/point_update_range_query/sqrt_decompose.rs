struct SqrtDecompose {
    blocks: Vec<i64>,
    arr: Vec<i64>,
}

impl SqrtDecompose {
    fn new(n: usize) -> Self {
        Self { blocks: vec![0; 450],
               arr: vec![0; n] }
    }

    fn pref_sum(&self, r: isize) -> i64 {
        (0..r / 450).map(|i| self.blocks[i as usize]).sum::<i64>()
        + ((r / 450) * 450..=r).map(|i| self.arr[i as usize])
                               .sum::<i64>()
    }

    fn rng_sum(&self, range: std::ops::RangeInclusive<isize>) -> i64 {
        self.pref_sum(*range.end()) - self.pref_sum(*range.start() - 1)
    }

    fn add(&mut self, i: usize, da: i64) {
        self.arr[i] += da;
        self.blocks[i / 450] += da;
    }

    fn set(&mut self, i: usize, da: i64) { self.add(i, da - self.arr[i]); }
}
