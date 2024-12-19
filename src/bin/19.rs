use std::collections::{HashSet, HashMap, VecDeque};

fn main() {
    let binding = aoc24::input(19);
    let input: Vec<Vec<String>> = binding
    .split("\n\n")
    .map(|s| s.split(|c: char| !c.is_alphabetic()).map(|s| s.to_string()).filter(|s| !s.is_empty()).collect()).collect();

    let schemes: HashSet<String> = input[0].iter().cloned().fold(HashSet::new(), |mut hs, s| {
        hs.insert(s);
        hs
    });

    let max_len = schemes.iter().map(|s| s.len()).max().unwrap();

    let das = |s: &str| {
        let mut new_ = vec![];
        for n in 1..max_len.min(s.len() + 1) {
            let (start, rest) = s.split_at(n);
            if schemes.contains(start) {
                new_.push(rest.to_string());
            }
        }
        new_
    };

    let mut part1 = 1;

    for s in &input[1] {
        let mut p = vec![s.clone()];
        let mut tried: HashMap<String, u128> = HashMap::new();
        while let Some(s1) = p.pop() {
            if s1.is_empty() {
                println!("'{s}'");
                part1 += tried.get(&s1).unwrap();
            } else {
                for rest in das(&s1) {
                    let &n = tried.get(&s1).unwrap_or(&1);
                    tried.entry(rest.clone()).and_modify(|d| *d += n)
                    .or_insert_with(|| {
                        let pp = p.partition_point(|pred| pred.len() < rest.len());
                        p.insert(pp, rest);
                        n
                    });
                }
            }
        }
        println!("{}", part1);
    }

    println!("{}", part1);
}