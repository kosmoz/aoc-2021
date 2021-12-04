mod error;
mod int_parser;
mod line;
mod lines;
mod parser;

use std::env::{self};

use int_parser::IntParser;

use crate::{lines::Lines, parser::Parser};

const MAX_INDEX: usize = 11;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let part = if argv.len() > 1 { &argv[1] } else { "part1" };
    let input = if argv.len() > 2 { &argv[2] } else { "input" };
    if part.eq("part2") {
        println!("part 2");
        part2(input.to_string())
    } else {
        println!("part 1");
        part1(input.to_string())
    }
}

fn part1(input: String) {
    let parser = Parser::new(&input).expect("cannot open file");
    let mut result = Lines::new();
    for line in parser {
        result.add(line.expect("cannot add line"));
    }
    println!("Result is: {}", result.prod())
}

fn part2(input: String) {
    let parser = IntParser::new(&input).expect("cannot open file");
    let lines: Vec<u16> = parser.map(|r| r.expect("cannot read line")).collect();
    println!(
        "Result is: {}",
        get_oxygen_rating(lines.clone()) as i32 * get_scrubber_rating(lines) as i32
    )
}

fn get_oxygen_rating(lines: Vec<u16>) -> u16 {
    let mut working = lines;
    for i in 0..(MAX_INDEX + 1) {
        working = filter_most_common(working, i);
        if working.len() == 1 {
            return working[0];
        }
    }
    return 0;
}

fn get_scrubber_rating(lines: Vec<u16>) -> u16 {
    let mut working = lines;
    for i in 0..(MAX_INDEX + 1) {
        working = filter_least_common(working, i);
        if working.len() == 1 {
            return working[0];
        }
    }
    return 0;
}

fn filter_most_common(lines: Vec<u16>, pos: usize) -> Vec<u16> {
    let mask = 1 << (MAX_INDEX - pos);
    let bit = get_most_common_bit(lines.clone(), pos) << (MAX_INDEX - pos);
    lines.into_iter().filter(|it| *it & mask == bit).collect()
}

fn filter_least_common(lines: Vec<u16>, pos: usize) -> Vec<u16> {
    let mask = 1 << (MAX_INDEX - pos);
    let bit = get_least_common_bit(lines.clone(), pos) << (MAX_INDEX - pos);
    lines.into_iter().filter(|it| *it & mask == bit).collect()
}

fn get_most_common_bit(lines: Vec<u16>, pos: usize) -> u16 {
    let (one, zero) = get_one_zero(lines, pos);
    (zero <= one) as u16
}

fn get_least_common_bit(lines: Vec<u16>, pos: usize) -> u16 {
    let (one, zero) = get_one_zero(lines, pos);
    (zero > one) as u16
}

fn get_one_zero(lines: Vec<u16>, pos: usize) -> (usize, usize) {
    let mask = 1 << (MAX_INDEX - pos);
    let yes = lines.iter().filter(|it| *it & mask > 0).count();
    (yes, lines.len() - yes)
}
