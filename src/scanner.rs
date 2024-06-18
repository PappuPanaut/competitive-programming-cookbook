#[derive(Default)]
struct Scanner {
    buf: Vec<String>,
}

impl Scanner {
    /// Returns white-space separated tokens one at a time
    ///
    /// # Panics
    ///
    /// Panics if there's an I/O error or if the token can't be parsed as T
    fn next<T: std::str::FromStr>(&mut self) -> T {
        if let Some(token) = self.buf.pop() {
            return token.parse().ok().expect("Failed parse");
        }

        let mut ln = String::new();
        std::io::stdin().read_line(&mut ln).expect("Failed read");

        self.buf = ln.split_whitespace().rev().map(String::from).collect();

        self.next()
    }
}
