use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::error::Error;

pub struct IntParser {
    reader: BufReader<File>,
}

impl IntParser {
    pub fn new(file_name: &str) -> Result<IntParser, Error> {
        let input_file = File::open(file_name.to_string())?;
        let input_reader = BufReader::new(input_file);
        Ok(IntParser {
            reader: input_reader,
        })
    }
}

impl Iterator for IntParser {
    type Item = Result<u16, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(0) => None,
            Ok(_) => match u16::from_str_radix(line.trim(), 2) {
                Ok(it) => Some(Ok(it)),
                Err(err) => Some(Err(Error::ParseError(err))),
            },
            Err(e) => Some(Err(Error::IoError(e))),
        }
    }
}
