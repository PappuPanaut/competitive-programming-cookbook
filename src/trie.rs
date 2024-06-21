#[derive(Default, Clone)]
struct Node {
    next: [usize; 26],
    end_count: u32,
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

    fn insert_str(&mut self, s: &str) {
        let mut v = 0;

        for c in s.chars().map(|c| (c as usize) - ('a' as usize)) {
            if self.t[v].next[c] == 0 {
                self.t[v].next[c] = self.t.len();
                self.t.push(Node::default());
            }

            v = self.t[v].next[c];
        }

        self.t[v].end_count += 1;
    }

    fn lcp(&self, s: &str) -> u32 {
        let mut lcp = 0;

        let (mut v, mut depth) = (0, 0);
        for c in s.chars().map(|c| (c as usize) - ('a' as usize)) {
            if self.t[v].end_count > 0
                || self.t[v].next.iter().map(|u| u.min(&1)).sum::<usize>() > 1
            {
                lcp = lcp.max(depth);
            }

            v = self.t[v].next[c];
            depth += 1;
        }

        if self.t[v].end_count > 1 || self.t[v].next.iter().map(|u| u.min(&1)).sum::<usize>() > 0 {
            lcp = lcp.max(depth);
        }

        lcp
    }
}
