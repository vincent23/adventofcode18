use std::io::{self, BufRead};

fn do_react(c1: char, c2: char) -> bool {
    c1.to_ascii_lowercase() == c2.to_ascii_lowercase()
        && ((c1.is_lowercase() && c2.is_uppercase()) || (c1.is_uppercase() && c2.is_lowercase()))
}

fn main() {
    let stdin = io::stdin();
    let orig_polymer = stdin.lock().lines().next().unwrap().unwrap();
    let mut shortest_length = std::usize::MAX;
    for unit_type in b'a'..b'z' {
        let mut polymer = orig_polymer.clone();
        polymer = polymer.replace(unit_type as char, "");
        polymer = polymer.replace((unit_type as char).to_ascii_uppercase(), "");
        loop {
            let index_result = {
                let mut second_char = polymer.chars();
                second_char.next();
                polymer
                    .char_indices()
                    .zip(second_char)
                    .filter_map(|((i, c1), c2)| if do_react(c1, c2) { Some(i) } else { None })
                    .next()
            };
            if let Some(index) = index_result {
                polymer.remove(index);
                polymer.remove(index);
            } else {
                break;
            }
        }
        if polymer.len() < shortest_length {
            shortest_length = polymer.len()
        }
    }
    println!("{}", shortest_length);
}
