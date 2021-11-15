pub struct Error {
    pub line: u32,
    pub msg: String,
}

impl Error {
    pub fn report(&self) {
        println!("[{}] Error: {}", self.line, self.msg);
    }
}
