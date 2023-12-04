use std::collections::{HashMap, HashSet};

fn main() {
    let mut counts = HashMap::<usize, usize>::new();
    include_str!("day04.txt")
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split(": ").nth(1).unwrap().split(" | ").map(|nums| {
                nums.split_ascii_whitespace()
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
        })
        .map(|mut it| -> (Vec<usize>, HashSet<usize>) {
            ((it.next().unwrap()), HashSet::from_iter(it.next().unwrap()))
        })
        .enumerate()
        .for_each(|(n, (winning_combination, card))| {
            let count = *counts.entry(n).and_modify(|e| *e += 1).or_insert(1);
            let wins = winning_combination
                .iter()
                .filter(|n| card.contains(n))
                .count();
            for i in 1..=wins {
                *counts.entry(n + i).or_insert(0) += count;
            }
        });
    println!("{}", counts.values().sum::<usize>());
}
