use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut count_two = 0;
    let mut count_three = 0;
    for line in lines {
        let mut char_count = [0; 26];
        for c in line.unwrap().bytes() {
            char_count[(c - b'a') as usize] += 1;
        }
        let mut any_two = 0;
        let mut any_three = 0;
        for count in char_count.iter() {
            match *count {
                2 => any_two |= 1,
                3 => any_three |= 1,
                _ => (),
            };
        }
        count_two += any_two;
        count_three += any_three;
    }
    println!("{}", count_two * count_three);
}
