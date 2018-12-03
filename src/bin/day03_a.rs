use std::io::{self, BufRead};

// from https://fgiesen.wordpress.com/2009/12/13/decoding-morton-codes/
fn part1by1(x: u32) -> u32 {
    let mut y = x & 0x0000ffff;
    y = (y ^ (y << 8)) & 0x00ff00ff;
    y = (y ^ (y << 4)) & 0x0f0f0f0f;
    y = (y ^ (y << 2)) & 0x33333333;
    y = (y ^ (y << 1)) & 0x55555555;
    return y;
}

fn encode_morton(x: u32, y: u32) -> u32 {
    (part1by1(y) << 1) + part1by1(x)
}

// https://graphics.stanford.edu/~seander/bithacks.html#RoundUpPowerOf2
fn next_power_of_2(x: u32) -> u32 {
    let mut y = x - 1;
    y |= y >> 1;
    y |= y >> 2;
    y |= y >> 4;
    y |= y >> 8;
    y |= y >> 16;
    y += 1;
    return y;
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

    let mut aabb = (std::u32::MAX, std::u32::MAX, 0u32, 0u32);

    for rect in rects.iter() {
        aabb = (
            std::cmp::min(rect.0, aabb.0),
            std::cmp::min(rect.1, aabb.1),
            std::cmp::max(rect.0 + rect.2, aabb.2),
            std::cmp::max(rect.1 + rect.3, aabb.3),
        );
    }

    // allocate square array for morton code index
    let width = next_power_of_2(aabb.2 - aabb.0);
    let height = next_power_of_2(aabb.3 - aabb.1);
    let square_length = std::cmp::max(width, height);
    let mut bitmap = vec![0u8; (square_length * square_length) as usize];

    // rasterize rectangles to bitmap
    for (x, y, w, h) in rects {
        let x_off = x - aabb.0;
        let y_off = y - aabb.1;
        for i in x_off..(x_off + w) {
            for j in y_off..(y_off + h) {
                let index = encode_morton(i, j) as usize;
                bitmap[index] = bitmap[index].saturating_add(1);
            }
        }
    }

    // count entries > 1
    let overlapped = bitmap.iter().filter(|c| **c > 1).count();
    println!("{}", overlapped);
}
