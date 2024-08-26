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
    arr: Vec<u32>,
    ans: Vec<u32>,
    cur_ans: u32,
    freq: HashMap<u32, u32>,
}

impl Mo {
    fn new(arr: &[u32], queries: &[Query]) -> Self {
        Self {
            queries: queries.to_vec(),
            arr: arr.to_vec(),
            ans: vec![0; queries.len()],
            cur_ans: 0,
            freq: HashMap::new(),
        }
    }

    fn add(&mut self, i: usize) {
        *self.freq.entry(self.arr[i]).or_insert(0) += 1;

        if self.freq[&self.arr[i]] == self.arr[i] {
            self.cur_ans += 1;
        } else if self.freq[&self.arr[i]] == self.arr[i] + 1 {
            self.cur_ans -= 1;
        }
    }

    fn del(&mut self, i: usize) {
        *self.freq.get_mut(&self.arr[i]).unwrap() -= 1;

        if self.freq[&self.arr[i]] == self.arr[i] {
            self.cur_ans += 1;
        } else if self.freq[&self.arr[i]] == self.arr[i] - 1 {
            self.cur_ans -= 1;
        }
    }

    fn solve(&mut self) {
        self.queries.sort();

        let (mut l, mut r) = (1, 0);
        for query in self.queries.clone() {
            while l > query.l {
                l -= 1;
                self.add(l);
            }

            while r < query.r {
                r += 1;
                self.add(r);
            }

            while l < query.l {
                self.del(l);
                l += 1;
            }

            while r > query.r {
                self.del(r);
                r -= 1;
            }

            self.ans[query.i] = self.cur_ans;
        }
    }
}
