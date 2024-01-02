use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let mut lines = include_str!("day08.txt")
        .lines()
        .filter(|line| !line.is_empty());
    let actions = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!("Arrrggh!"),
        })
        .collect_vec();
    let n = actions.len();
    let map = lines.fold(HashMap::<&str, [&str; 2]>::new(), |mut hm, x| {
        let (from, to) = x.split_once(" = ").unwrap();
        let it = to.split_once(", ").unwrap();
        let to_left = &it.0[1..=3];
        let to_right = &it.1[0..3];
        hm.insert(from, [to_left, to_right]);
        hm
    });
    let mut step = 0;
    let mut at = "AAA";
    while at != "ZZZ" {
        at = map.get(at).unwrap()[actions[step % n]];
        step += 1;
    }
    println!("{}", step);
}
