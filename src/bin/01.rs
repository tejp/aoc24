use std::collections::HashMap;

fn insert_sorted(v: &mut Vec<u64>, n: u64) {
    let i = match v.binary_search(&n) {
        Ok(n) | Err(n) => n,
    };
    v.insert(i, n);
}

fn main() {
    let input = std::fs::read_to_string("input/01").unwrap();

    let (mut left, mut right) = (vec![], vec![]);

    for line in input.lines() {
        let mut nums = line.split("   ");

        insert_sorted(&mut left, nums.next().unwrap().parse().unwrap());
        insert_sorted(&mut right, nums.next().unwrap().parse().unwrap());
    }

    let total: u64 = left
        .iter()
        .by_ref()
        .zip(right.iter().by_ref())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();

    println!("{}", total);

    let right_count = right.into_iter().fold(HashMap::new(), |mut map, val| {
        map.entry(val).and_modify(|frq| *frq += 1u64).or_insert(1);
        map
    });

    let similarity: u64 = left
        .iter()
        .map(|n| n * right_count.get(n).unwrap_or(&0))
        .sum();

    println!("{}", similarity);
}
