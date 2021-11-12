pub struct Scanner {
    pub source: String,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner { source }
    }

    pub fn scan_tokens(&self) {
        println!("{:?}", self.source);
    }
}
