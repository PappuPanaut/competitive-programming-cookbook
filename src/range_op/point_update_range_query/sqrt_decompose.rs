struct SqrtDecompose {
    blocks: Vec<i64>,
    arr: Vec<i64>,
}

impl SqrtDecompose {
    fn with_len(n: usize) -> Self {
        Self { blocks: vec![0; 450],
               arr: vec![0; n] }
    }

    fn prefix_sum(&self, r: isize) -> i64 {
        (0..r / 450).map(|i| self.blocks[i as usize]).sum::<i64>()
        + ((r / 450) * 450..=r).map(|i| self.arr[i as usize])
                               .sum::<i64>()
    }

    fn range_sum(&self, range: std::ops::RangeInclusive<isize>) -> i64 {
        self.prefix_sum(*range.end()) - self.prefix_sum(*range.start() - 1)
    }

    fn increment(&mut self, i: usize, da: i64) {
        self.arr[i] += da;
        self.blocks[i / 450] += da;
    }

    fn set(&mut self, i: usize, da: i64) { self.increment(i, da - self.arr[i]); }
}
