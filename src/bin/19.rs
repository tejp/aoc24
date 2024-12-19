use std::collections::{HashMap, HashSet};

fn main() {
    let input: Vec<Vec<String>> = aoc24::input(19)
        .trim()
        .split("\n\n")
        .map(|s| {
            s.split(|c: char| !c.is_alphabetic())
                .map(|s| s.to_string())
                .collect()
        })
        .collect();

    let schemes: HashSet<&str> = HashSet::from_iter(input[0].iter().map(String::as_str));

    let max_len = schemes.iter().map(|s| s.len()).max().unwrap();

    let mut part1 = 0;
    let mut part2 = 0;

    for s in &input[1] {
        let mut todo = vec![s.as_str()];
        let mut visited: HashMap<&str, u64> = HashMap::from([(s.as_str(), 1)]);
        while let Some(s1) = todo.pop() {
            let &n = visited.get(s1).unwrap();
            if s1.is_empty() {
                part1 += 1;
                part2 += n;
            } else {
                for len in 1..max_len.min(s1.len()) + 1 {
                    let (start, rest) = s1.split_at(len);
                    if schemes.contains(start) {
                        if let Some(d) = visited.get_mut(rest) {
                            *d += n;
                        } else {
                            visited.insert(rest, n);
                            let pp = todo.partition_point(|&pred| pred.len() < rest.len());
                            todo.insert(pp, rest);
                        }
                    }
                }
            }
        }
    }

    println!("{}", part1);
    println!("{}", part2);
}
