#[derive(Default)]
struct Scanner {
    buf: Vec<String>,
}

impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        if let Some(token) = self.buf.pop() {
            return token.parse().ok().expect("Parse fail");
        }

        let mut ln = String::new();
        std::io::stdin().read_line(&mut ln).expect("Read fail");
        self.buf = ln.split_whitespace().rev().map(String::from).collect();

        self.next()
    }
}
