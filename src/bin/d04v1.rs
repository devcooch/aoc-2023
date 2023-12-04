use std::collections::HashSet;

fn main() {
    println!(
        "{}",
        include_str!("day04.txt")
            .lines()
            .filter(|l| !l.is_empty())
            .map(
                |l| l.split(": ").nth(1).unwrap().split(" | ").map(|nums| nums
                    .split_ascii_whitespace()
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect::<Vec<_>>())
            )
            .map(|mut it| -> (Vec<u32>, HashSet<u32>) {
                ((it.next().unwrap()), HashSet::from_iter(it.next().unwrap()))
            })
            .map(|(win, card)| win.iter().filter(|n| card.contains(n)).count())
            .filter(|c| *c > 0)
            .map(|c| 2u32.pow((c - 1) as u32))
            .sum::<u32>()
    );
}
