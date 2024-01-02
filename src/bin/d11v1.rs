use std::iter;

fn main() {
    let data = include_str!("day11.txt");
    let map: Vec<Vec<bool>> = data
        .lines()
        .map(|x| x.chars().map(|y| y == '#').collect())
        .collect();
    let h = map.len();
    let w = map[0].len();
    let empty_rows = map
        .iter()
        .enumerate()
        .filter(|(i, row)| row.iter().all(|x| !x))
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
}
