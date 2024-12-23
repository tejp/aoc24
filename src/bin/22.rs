use std::collections::{HashMap, HashSet};

#[inline(always)]
fn mix_prune(secret_num: &mut u64, other: u64) {
    *secret_num ^= other;
    *secret_num %= 16777216;
}

#[inline(always)]
fn next_num(secret_num: &mut u64) {
    let mut new_num = *secret_num * 64;
    mix_prune(secret_num, new_num);
    new_num = *secret_num / 32;
    mix_prune(secret_num, new_num);
    new_num = *secret_num * 2048;
    mix_prune(secret_num, new_num);
}

fn main() {
    let mut input: Vec<u64> = aoc24::input(22)
        .split(|c: char| !c.is_numeric())
        .filter_map(|s| s.parse().ok())
        .collect();

    let mut hm: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();

    for n in input.iter_mut() {
        let mut value_diff = vec![];
        let mut prev_value = *n as i64 % 10;
        for _ in 0..2000 {
            next_num(n);
            let value = *n as i64 % 10;
            value_diff.push((value, value - prev_value));
            prev_value = value;
        }
        let mut added = HashSet::new();
        value_diff
            .iter()
            .zip(value_diff[1..].iter())
            .zip(value_diff[2..].iter())
            .zip(value_diff[3..].iter())
            .for_each(|(((vd1, vd2), vd3), vd4)| {
                let key = (vd1.1, vd2.1, vd3.1, vd4.1);
                if added.insert(key) {
                    hm.entry(key)
                        .and_modify(|price| *price += vd4.0)
                        .or_insert(vd4.0);
                }
            });
    }

    let part1: u64 = input.iter().sum();
    let part2 = hm.values().max().unwrap();

    println!("{}", part1);
    println!("{:?}", part2);
}
