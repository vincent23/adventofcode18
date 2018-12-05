extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead};

enum RecordType {
    FallsAsleep,
    WakesUp,
    BeginsShift(i32),
}

struct Record {
    time: i32,
    record_type: RecordType,
}

fn parse_timestamp(month_s: &str, day_s: &str, hour_s: &str, minute_s: &str) -> i32 {
    let month = i32::from_str_radix(month_s, 10).unwrap();
    let day = i32::from_str_radix(day_s, 10).unwrap();
    let hour = i32::from_str_radix(hour_s, 10).unwrap();
    let minute = i32::from_str_radix(minute_s, 10).unwrap();
    ((month * 31 + day) * 24 + hour) * 60 + minute
}

fn main() {
    let stdin = io::stdin();
    let date_re = Regex::new(
        r"(?x)
\[
\d{4} # year
-
(\d{2}) # month
-
(\d{2}) # day
\s
(\d{2}) # hour
:
(\d{2}) # minute
\]
\s
(falls|wakes|Guard\s\#(\d+))
",
    ).unwrap();

    let mut records: Vec<_> = stdin
        .lock()
        .lines()
        .map(|l| {
            let line = l.unwrap();

            let caps = date_re.captures(&line).unwrap();
            Record {
                time: parse_timestamp(&caps[1], &caps[2], &caps[3], &caps[4]),
                record_type: match &caps[5] {
                    "falls" => RecordType::FallsAsleep,
                    "wakes" => RecordType::WakesUp,
                    _ => RecordType::BeginsShift(i32::from_str_radix(&caps[6], 10).unwrap()),
                },
            }
        }).collect();
    records.sort_unstable_by(|a, b| a.time.cmp(&b.time));
    let mut current_guard = -1;
    let mut sleep_start = -1;
    let mut minutes_slept: HashMap<i32, Vec<i32>> = HashMap::new();
    for record in records {
        match record.record_type {
            RecordType::FallsAsleep => {
                assert!(sleep_start == -1);
                assert!(current_guard >= 0);
                sleep_start = record.time % 60;
            }
            RecordType::WakesUp => {
                assert!(sleep_start >= 0);
                assert!(current_guard >= 0);
                let sleep_end = record.time % 60;
                let mut minutes_guard = minutes_slept.get_mut(&current_guard).unwrap();
                for time in sleep_start..sleep_end {
                    minutes_guard[time as usize] += 1;
                }
                sleep_start = -1;
            }
            RecordType::BeginsShift(id) => {
                assert!(sleep_start == -1);
                current_guard = id;
                if !minutes_slept.contains_key(&id) {
                    minutes_slept.insert(id, vec![0; 60]);
                }
            }
        }
    }

    let mut max_total = 0;
    let mut max_minute = 0;
    let mut max_guard = 0;
    for (guard, minutes) in minutes_slept.iter() {
        let total: i32 = minutes.iter().sum();
        if total > max_total {
            max_total = total;
            max_guard = *guard;
            max_minute = minutes.iter().enumerate().max_by_key(|x| x.1).unwrap().0 as i32;
        }
    }
    println!(
        "a {}: {} * {}",
        max_guard * max_minute,
        max_guard,
        max_minute
    );

    max_total = 0;
    for (guard, minutes) in minutes_slept.iter() {
        let (minute, count) = minutes.iter().enumerate().max_by_key(|x| x.1).unwrap();
        if *count > max_total {
            max_total = *count;
            max_guard = *guard;
            max_minute = minute as i32;
        }
    }
    println!(
        "b {}: {} * {}",
        max_guard * max_minute,
        max_guard,
        max_minute
    );
}
