#[derive(Default, Clone)]
struct Node {
    next: [u32; 2],
}

struct Trie {
    t: Vec<Node>,
}

impl Trie {
    fn new() -> Self {
        Self {
            t: vec![Node::default(); 1],
        }
    }

    fn insert(&mut self, x: u64) {
        let mut v = 0;

        for xb in (0..41).rev().map(|b| (x >> b) as usize & 1) {
            if self.t[v].next[xb] == 0 {
                self.t[v].next[xb] = self.t.len() as _;
                self.t.push(Node::default());
            }

            v = self.t[v].next[xb] as _;
        }
    }

    fn max_xor_with(&self, x: u64) -> u64 {
        let mut res = 0;

        let mut v = 0;
        for xb in (0..41).rev().map(|b| (x >> b) as usize & 1) {
            res *= 2;

            match self.t[v].next[1 ^ xb] > 0 {
                true => {
                    res += 1;
                    v = self.t[v].next[1 ^ xb] as _;
                }
                false => v = self.t[v].next[xb] as _,
            }
        }

        res
    }
}
