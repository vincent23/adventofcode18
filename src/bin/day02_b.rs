use std::io::{self, BufRead};

/// Finds the index of the only differing char if `a` and `b` only differ in exactly one position.
fn differing_char(a: &str, b: &str) -> Option<usize> {
    if a.len() != b.len() {
        return None;
    }
    let mut index: Option<usize> = None;

    for i in 0..a.len() {
        if a.as_bytes()[i] == b.as_bytes()[i] {
            continue;
        }

        if index.is_none() {
            index = Some(i);
        } else {
            return None;
        }
    }
    return index;
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    // try all pairs (O(n^2))
    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            let differing = differing_char(&lines[i], &lines[j]);
            if let Some(index) = differing {
                let mut common_letters = lines[i].clone();
                common_letters.remove(index);
                println!("{}", common_letters);
                return;
            }
        }
    }
}
