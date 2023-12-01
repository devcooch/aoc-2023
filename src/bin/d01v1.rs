fn main() {
    println!(
        "{}",
        include_str!("day01.txt")
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().filter_map(|c| c.to_digit(10)).peekable())
            .map(|mut it| it.peek().unwrap() * 10 + it.last().unwrap())
            .sum::<u32>()
    )
}
