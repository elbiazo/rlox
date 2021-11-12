pub struct Error {
    pub line: u32,
    pub err: String,
    pub msg: String,
}

impl Error {
    pub fn report(&self) {
        println!("[{}] Error {}: {}", self.line, self.err, self.msg);
    }
}
