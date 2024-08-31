#[derive(Clone, Eq, PartialEq)]
struct Query {
    l: usize,
    r: usize,
    i: usize,
}

impl PartialOrd for Query {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Query {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.l / 450, self.r).cmp(&(other.l / 450, other.r))
    }
}

struct Mo {
    queries: Vec<Query>,
    arr: Vec<u64>,
    ans: Vec<u64>,
    pow: u64,
    freq: Vec<u64>,
}

impl Mo {
    fn new(arr: &[u64], queries: &[Query]) -> Self {
        Self {
            queries: queries.to_vec(),
            arr: arr.to_vec(),
            ans: vec![0; queries.len()],
            pow: 0,
            freq: vec![0; 1_000_001],
        }
    }

    fn add(&mut self, ai: u64) {
        self.freq[ai as usize] += 1;
        self.pow += ai * (2 * self.freq[ai as usize] - 1);
    }

    fn del(&mut self, ai: u64) {
        self.pow -= ai * (2 * self.freq[ai as usize] - 1);
        self.freq[ai as usize] -= 1;
    }

    fn solve(&mut self) {
        self.queries.sort();

        let (mut l, mut r) = (self.queries[0].l, self.queries[0].l);
        self.add(self.arr[l]);
        for query in self.queries.clone() {
            while l > query.l {
                l -= 1;
                self.add(self.arr[l]);
            }

            while r < query.r {
                r += 1;
                self.add(self.arr[r]);
            }

            while l < query.l {
                self.del(self.arr[l]);
                l += 1;
            }

            while r > query.r {
                self.del(self.arr[r]);
                r -= 1;
            }

            self.ans[query.i] = self.pow;
        }
    }
}
