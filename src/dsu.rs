#[derive(Default)]
struct Dsu {
    pa: std::collections::HashMap<u32, u32>,
}

impl Dsu {
    fn rep(&mut self, u: u32) -> u32 {
        if !self.pa.contains_key(&u) {
            return u;
        }

        let ru = self.rep(self.pa[&u]);
        self.pa.insert(u, ru);

        ru
    }

    fn merge(&mut self, u: u32, v: u32) {
        let (ru, rv) = (self.rep(u), self.rep(v));

        if ru != rv {
            self.pa.insert(ru, rv);
        }
    }
}
