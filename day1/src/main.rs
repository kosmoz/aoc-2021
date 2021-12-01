use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> std::io::Result<()> {
    let input_file = File::open("input")?;
    let mut input_reader = BufReader::new(input_file);

    let mut line = String::new();
    let mut a = None::<i64>;
    let mut b = None::<i64>;
    let mut prev_sum = None::<i64>;
    let mut count: i64 = 0;

    while input_reader.read_line(&mut line)? != 0 {
        let c = line
            .trim()
            .parse::<i64>()
            .expect(&format!("expected a number but found \"{}\"", line.trim()));

        if a.is_some() {
            let current_sum = a.unwrap() + b.unwrap() + c;
            if prev_sum.is_none() {
                println!("{} (N/A - no previous measurements)", current_sum);
            } else {
                if current_sum > prev_sum.unwrap() {
                    println!("{} (increased)", current_sum);
                    count += 1;
                } else {
                    println!("{} (decreased)", current_sum);
                }
            }
            prev_sum = Some(current_sum);
        }

        a = b;
        b = Some(c);
        line = String::new();
    }

    println!(
        "There were {} measurements that were larger than the previous measurement.",
        count
    );

    Ok(())
}
