use once_cell::sync::Lazy;
use regex::Regex;

fn str_to_num(s: &str) -> u32 {
    match s {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => panic!("Achtung, bitte!"),
    }
}

fn calculate_line(l: &str) -> u32 {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(
            r"((one|1)|(two|2)|(three|3)|(four|4)|(five|5)|(six|6)|(seven|7)|(eight|8)|(nine|9))",
        )
        .unwrap()
    });
    let mut n = 0usize;
    let mut first: Option<u32> = None;
    let mut last: Option<u32> = None;
    while let Some(m) = RE.find_at(l, n) {
        if first.is_none() {
            first = Some(str_to_num(m.as_str()));
        }
        last = Some(str_to_num(m.as_str()));
        n += 1;
    }
    first.unwrap() * 10 + last.unwrap()
}

fn main() {
    println!(
        "{}",
        include_str!("day01.txt")
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| calculate_line(line))
            .sum::<u32>()
    );
}
