mod error;
mod line;
mod lines;
mod parser;

use crate::{lines::Lines, parser::Parser};

fn main() {
    let parser = Parser::new("input").expect("cannot open file");
    let mut result = Lines::new();
    for line in parser {
        result.add(line.expect("cannot add line"));
    }
    println!("Result is: {}", result.prod())
}
