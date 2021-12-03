mod error;
mod operation;
mod parser;
mod submarine;

use parser::Parser;
use submarine::Submarine;

fn main() {
    let parser = Parser::new("input").expect("cannot open file");
    let mut sub = Submarine::new();
    for op in parser {
        sub = sub.apply(op.expect("cannot do operation"));
    }
    println!("Result is {}.", sub.position())
}
