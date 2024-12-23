use std::collections::{BTreeSet, HashMap, HashSet};

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

    let mut hm: HashMap<&str, BTreeSet<&str>> = HashMap::new();

    let mut all_sets = HashSet::new();

    for [l, r] in &input {
        hm.entry(l)
            .and_modify(|v| {
                v.insert(r);
            })
            .or_insert(BTreeSet::from([l.as_str(), r]));
        hm.entry(r)
            .and_modify(|v| {
                v.insert(l);
            })
            .or_insert(BTreeSet::from([l.as_str(), r]));
        let v = BTreeSet::from_iter([l.as_str(), r]);
        all_sets.insert(v);
    }

    // let mut part2 = BTreeSet::new();

    // for (_, set) in &hm {
    //     let mut interset = set.clone();

    //     loop {
    //         let mut remo: HashMap<&String, i32> = HashMap::new();

    //         for k in interset {
    //             let mut set2 = hm.get(k).unwrap();
    //             for &e in set.difference(set2) {
    //                 remo.entry(e).and_modify(|c| *c += 1).or_insert(1);
    //             }
    //         }
    //         if remo.is_empty() {
    //             break;
    //         }
    //     }

    //     //let c: BTreeSet<_> = l_set.intersection(r_set).chain([&l, &r]).collect();
    //     if interset.len() > part2.len() {
    //         println!("{:?}", interset);
    //         part2 = interset.len();
    //     }

    // }

    // let mut part1 = 0;

    // for (&e, v) in &hm {
    //     let combs = v
    //         .iter()
    //         .flat_map(|&e1| v.iter().map(move |&e2| (e1, e2)))
    //         .collect();
    //     for (l, r) in combs {
    //         if hm.get(l).unwrap().contains(&r)
    //             && (e.starts_with("t") || l.starts_with("t") || r.starts_with("t"))
    //         {
    //             part1 += 1;
    //         }
    //     }
    // }

    // println!("{}", part1);

    for (&e, s1) in &hm {
        let v = all_sets
            .iter()
            .filter(|s2| s1.is_superset(s2))
            .cloned()
            .map(|mut s2| {
                s2.insert(e);
                s2
            })
            .collect::<Vec<_>>();
        all_sets.extend(v);
    }

    let part1 = all_sets
        .iter()
        .filter(|s| s.len() == 3 && s.iter().any(|s| s.starts_with("t")))
        .count();

    let part2 = all_sets
        .iter()
        .max_by(|v1, v2| v1.len().cmp(&v2.len()))
        .unwrap()
        .iter()
        .cloned()
        .collect::<Vec<&str>>()
        .join(",");

    println!("{}", part1);
    println!("{}", part2);
}
