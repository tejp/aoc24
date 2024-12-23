use std::collections::{HashMap, HashSet};

fn main() {
    let input: Vec<[String; 2]> = aoc24::input(23)
        .lines()
        .map(|s| {
            s.split(|c: char| !c.is_alphabetic())
                .map(str::to_string)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect();

    let mut hm: HashMap<&String, Vec<&String>> = HashMap::new();

    let mut hs = HashSet::new();

    for [l, r] in &input {
        hm.entry(l).and_modify(|v| v.push(r)).or_insert(vec![r]);
        hm.entry(r).and_modify(|v| v.push(l)).or_insert(vec![l]);
        let mut v = vec![l, r];
        v.sort();
        hs.insert(v);
    }

    let mut part1 = 0;

    for (&e, v) in &hm {
        let combs = v.iter().enumerate().flat_map(|(i, &e1)| v[i+1..].iter().map(move |&e2|(e1,e2)));
        for (l, r) in combs {
            if hm.get(l).unwrap().contains(&r) && (e.starts_with("t") || l.starts_with("t") || r.starts_with("t")) {
                part1 += 1;
            }
        }
    }

    println!("{}", part1);

    for (&e, v) in &hm {
        let mut to_add = vec![];
        for s in &hs {
            if s.iter().all(|s2| v.contains(s2)) {
                let mut v = s.clone();
                v.push(e);
                v.sort();
                to_add.push(v);
            }
        }
        hs.extend(to_add);
    }

    let part2 = hs.iter().max_by(|v1, v2| v1.len().cmp(&v2.len())).unwrap();

    println!("{:?}", part2);
}
