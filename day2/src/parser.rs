use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{error::Error, operation::Operation};

pub struct Parser {
    reader: BufReader<File>,
}

impl Parser {
    pub fn new(file_name: &str) -> Result<Parser, Error> {
        let input_file = File::open(file_name.to_string())?;
        let input_reader = BufReader::new(input_file);
        Ok(Parser {
            reader: input_reader,
        })
    }
}

impl Iterator for Parser {
    type Item = Result<Operation, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(0) => None,
            Ok(_) => Some(Operation::parse(line)),
            Err(e) => Some(Err(Error::IoError(e))),
        }
    }
}
