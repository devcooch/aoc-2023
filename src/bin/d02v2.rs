use std::cmp::max;
use std::collections::HashMap;

fn main() {
    println!(
        "{}",
        include_str!("day02.txt")
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line
                .split(": ")
                .nth(1)
                .unwrap()
                .split("; ")
                .flat_map(|set| set
                    .split(", ")
                    .map(|cube| cube.split(' ').collect::<Vec<_>>())
                    .map(|v| (v[0].parse::<usize>().unwrap(), v[1].chars().next().unwrap()))
                    .collect::<Vec<_>>())
                .fold(HashMap::new(), |mut game, x| {
                    let _ = *game
                        .entry(x.1)
                        .and_modify(|n| *n = max(*n, x.0))
                        .or_insert(x.0);
                    game
                }))
            .map(|hm| hm.get(&'r').unwrap() * hm.get(&'g').unwrap() * hm.get(&'b').unwrap())
            .sum::<usize>()
    );
}
