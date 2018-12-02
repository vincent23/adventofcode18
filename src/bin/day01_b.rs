use std::collections::HashSet;
use std::io::{self, BufRead};

// this is a naive/analog solution. it worked on the given input, but can require many iterations
// and hashset entries.

fn main() {
    let mut freq = 0;
    let mut seen_freqs = HashSet::new();
    seen_freqs.insert(freq);

    let stdin = io::stdin();
    let changes: Vec<_> = stdin
        .lock()
        .lines()
        .map(|line| i32::from_str_radix(&line.unwrap(), 10).unwrap())
        .collect();

    for change in changes.iter().cycle() {
        freq += change;
        if seen_freqs.contains(&freq) {
            break;
        }
        seen_freqs.insert(freq);
    }
    println!("{}", freq);
}
