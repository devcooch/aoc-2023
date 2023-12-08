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
                                .map(|c| "0123456789TJQKA".find(c).unwrap())
                                .collect::<Vec<_>>(),
                            tuple.1.parse::<usize>().unwrap(),
                            tuple.0.chars().fold(HashMap::new(), |mut counts, c| {
                                *counts.entry(c).or_insert(0usize) += 1;
                                counts
                            }),
                        )
                    })
                    .map(|(hand, bid, counts)| {
                        (
                            hand,
                            bid,
                            counts.values().max().unwrap() * 10
                                + counts.values().filter(|&x| *x == 2).count(),
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
