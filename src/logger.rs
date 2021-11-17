pub struct Error {
    pub line: usize,
    pub msg: String,
}

pub trait Report {
    fn report(&self, err: &Error);
}
