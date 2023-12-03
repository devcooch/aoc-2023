use itertools::Itertools;

fn main() {
    println!(
        "{:?}",
        include_str!("day02.txt")
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line
                .split(": ")
                .skip(1)
                .next()
                .unwrap()
                .split("; ")
                .map(|set| set
                    .split(", ")
                    .map(|cube| cube.split(" ").collect_vec())
                    .map(|v| (v[0].parse::<usize>().unwrap(), v[1].chars().next().unwrap()))
                    .all(|cube| (cube.1 == 'r' && cube.0 <= 12)
                        || (cube.1 == 'g' && cube.0 <= 13)
                        || (cube.1 == 'b' && cube.0 <= 14)))
                .all(|set| set))
            .enumerate()
            .map(|(no, possible)| if possible { no + 1 } else { 0 })
            .sum::<usize>()
    );
}
