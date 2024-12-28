use std::collections::{HashMap, HashSet};

fn main() {
    let mut input: Vec<u64> = aoc24::input(22)
        .split(|c: char| !c.is_numeric())
        .filter_map(|s| s.parse().ok())
        .collect();

    let mut best: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();

    for n in input.iter_mut() {
        let mut vdiff = vec![];
        for _ in 0..2000 {
            let prev_value = *n as i64 % 10;
            *n = *n ^ *n * 64 % 16777216;
            *n = *n ^ *n / 32 % 16777216;
            *n = *n ^ *n * 2048 % 16777216;
            let value = *n as i64 % 10;
            vdiff.push((value, value - prev_value));
        }
        let mut added = HashSet::new();
        for vd in (0..vdiff.len() - 4).map(|i| &vdiff[i..]) {
            let key = (vd[0].1, vd[1].1, vd[2].1, vd[3].1);
            if added.insert(key) {
                best.entry(key)
                    .and_modify(|price| *price += vd[3].0)
                    .or_insert(vd[3].0);
            }
        }
    }

    let part1: u64 = input.iter().sum();
    let part2 = best.values().max().unwrap();

    println!("{}", part1);
    println!("{}", part2);
}
