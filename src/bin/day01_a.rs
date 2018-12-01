use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let freq: i32 = stdin
        .lock()
        .lines()
        .map(|line| i32::from_str_radix(&line.unwrap(), 10).unwrap())
        .sum();
    println!("{}", freq);
}
