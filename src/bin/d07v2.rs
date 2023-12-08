use itertools::Itertools;
use std::{
    cmp::{Ord, Ordering},
    collections::HashMap,
};

fn main() {
    println!(
        "{}",
        include_str!("day07.txt")
            .lines()
            .filter(|line| !line.is_empty())
            .flat_map(|line| {
                line.split_ascii_whitespace()
                    .tuples::<(_, _)>()
                    .map(|tuple| {
                        (
                            tuple
                                .0
                                .chars()
                                .map(|c| "0J23456789TQKA".find(c).unwrap())
                                .collect::<Vec<_>>(),
                            tuple.1.parse::<usize>().unwrap(),
                            tuple.0.chars().filter(|c| *c != 'J').fold(
                                HashMap::new(),
                                |mut counts, c| {
                                    *counts.entry(c).or_insert(0usize) += 1;
                                    counts
                                },
                            ),
                            tuple.0.chars().filter(|c| *c == 'J').count(),
                        )
                    })
                    .map(|(hand, bid, counts, jokers)| {
                        (
                            hand,
                            bid,
                            counts.values().max().unwrap_or(&0) * 10
                                + counts.values().filter(|&x| *x == 2).count(),
                            jokers,
                        )
                    })
                    .map(|(hand, bid, rating, jokers)| {
                        (
                            hand,
                            bid,
                            match (rating, jokers) {
                                (21, 1 | 2 | 3) | (22, 1) => rating + jokers * 10 - 1,
                                (_, _) => rating + jokers * 10,
                            },
                        )
                    })
            })
            .collect::<Vec<(_, _, _)>>()
            .into_iter()
            .sorted_by(|(hand1, _, rating1), (hand2, _, rating2)| {
                let cmp = Ord::cmp(&rating1, &rating2);
                if cmp == Ordering::Equal {
                    Ord::cmp(&hand1, &hand2)
                } else {
                    cmp
                }
            })
            .enumerate()
            .map(|(n, (_, bid, _))| (n + 1) * bid)
            .sum::<usize>()
    );
}
