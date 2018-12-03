use std::io::{self, BufRead};

type Rect = (u32, u32, u32, u32);

fn rect_overlap(a: &Rect, b: &Rect) -> bool {
    if !(a.0 <= (b.0 + b.2) && b.0 <= (a.0 + a.2)) {
        return false;
    }
    if !(a.1 <= (b.1 + b.3) && b.1 <= (a.1 + a.3)) {
        return false;
    }
    return true;
}

fn main() {
    let stdin = io::stdin();
    let rects: Vec<_> = stdin
        .lock()
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let mut string_iter = line.split(|c| " ,:x".contains(c));
            string_iter.next(); // skip number
            string_iter.next(); // skip @
            let x = u32::from_str_radix(string_iter.next().unwrap(), 10).unwrap();
            let y = u32::from_str_radix(string_iter.next().unwrap(), 10).unwrap();
            string_iter.next(); // skip space
            let w = u32::from_str_radix(string_iter.next().unwrap(), 10).unwrap();
            let h = u32::from_str_radix(string_iter.next().unwrap(), 10).unwrap();
            (x, y, w, h)
        }).collect();

    for (i, rect) in rects.iter().enumerate() {
        if !rects
            .iter()
            .enumerate()
            .any(|(j, r)| if i == j { false } else { rect_overlap(rect, r) })
        {
            println!("{}", i + 1);
            return;
        }
    }
}
