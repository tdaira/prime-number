use std::io::BufRead;
use std::io::stdin;

pub trait ValueReader {
    fn read<T: std::str::FromStr>(&mut self) -> Option<T>;
    fn read_vec<T: std::str::FromStr>(&mut self) -> Vec<T>;
}

pub struct StdReader {}

impl ValueReader for StdReader {
    fn read<T: std::str::FromStr>(&mut self) -> Option<T> {
        let mut s = String::new();
        stdin().lock().read_line(&mut s).unwrap();
        s.trim().parse().ok()
    }

    fn read_vec<T: std::str::FromStr>(&mut self) -> Vec<T> {
        let mut s = String::new();
        stdin().lock().read_line(&mut s).unwrap();
        s.trim().split_whitespace()
            .map(|e| e.parse().ok().unwrap()).collect()
    }
}

impl StdReader {
    pub fn new() -> StdReader { StdReader{} }
}

pub struct StringReader<'a> {
    input: &'a [u8]
}

impl<'a> ValueReader for StringReader<'a> {
    fn read<T: std::str::FromStr>(&mut self) -> Option<T> {
        let mut s = String::new();
        self.input.read_line(&mut s).unwrap();
        s.trim().parse().ok()
    }

    fn read_vec<T: std::str::FromStr>(&mut self) -> Vec<T> {
        let mut s = String::new();
        self.input.read_line(&mut s).unwrap();
        s.trim().split_whitespace()
            .map(|e| e.parse().ok().unwrap()).collect()
    }
}

impl<'a> StringReader<'a> {
    pub fn new(input: &[u8]) -> StringReader {
        StringReader{ input }
    }
}
