use std::io::Write;
use std::io::Error;
use std::io::stdout;

pub trait ValueWriter {
    fn write<T: ToString>(&mut self, v: T) -> Result<(), Error>;
    fn write_vec<T: ToString>(&mut self, vec: Vec<T>) -> Result<(), Error>;
}

pub struct StdWriter{}

impl ValueWriter for StdWriter {
    fn write<T: ToString>(&mut self, v: T) -> Result<(), Error> {
        let stdout = stdout();
        let mut handle = stdout.lock();
        handle.write((v.to_string() + &"\n".to_string()).as_bytes())?;
        handle.flush()
    }

    fn write_vec<T: ToString>(&mut self, vec: Vec<T>) -> Result<(), Error> {
        if vec.len() < 1 {
            return Ok(())
        }
        let stdout = stdout();
        let mut handle = stdout.lock();
        handle.write(vec[0].to_string().as_bytes())?;
        for v in &vec[1..] {
            handle.write((" ".to_string() + &v.to_string()).as_bytes())?;
        }
        handle.write("\n".as_bytes())?;
        handle.flush()
    }
}

impl StdWriter {
    pub fn new() -> StdWriter { StdWriter{} }
}

pub struct StringWriter<'a> {
    pub output: &'a mut Vec<u8>
}

impl<'a> ValueWriter for StringWriter<'a> {
    fn write<T: ToString>(&mut self, v: T) -> Result<(), Error> {
        self.output.write((v.to_string() + &"\n".to_string()).as_bytes())?;
        return self.output.flush();
    }

    fn write_vec<T: ToString>(&mut self, vec: Vec<T>) -> Result<(), Error> {
        if vec.len() < 1 {
            return Ok(())
        }
        self.output.write(vec[0].to_string().as_bytes())?;
        for v in &vec[1..] {
            self.output.write((" ".to_string() + &v.to_string()).as_bytes())?;
        }
        self.output.write("\n".as_bytes())?;
        return self.output.flush();
    }
}

impl<'a> StringWriter<'a> {
    pub fn new(output: &mut Vec<u8>) -> StringWriter {
        StringWriter{ output }
    }
}
