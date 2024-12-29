use std::collections::{BTreeSet, HashMap, VecDeque};

fn main() {
    let input: Vec<Vec<String>> = aoc24::input(24)
        .split("\n\n")
        .map(|s| {
            s.split(|c: char| !c.is_alphanumeric())
                .filter_map(|s| (!s.is_empty()).then_some(s.to_string()))
                .collect()
        })
        .collect();

    let mut regs: HashMap<&String, u64> = input[0]
        .chunks_exact(2)
        .map(|c| (&c[0], c[1].parse().unwrap()))
        .collect();

    let mut work: VecDeque<&[String]> = input[1].chunks_exact(4).collect();

    while let Some(a @ [l, op, r, s]) = work.pop_front() {
        if let (Some(&l), Some(&r)) = (regs.get(l), regs.get(r)) {
            let res = match op.as_str() {
                "AND" => l & r,
                "OR" => l | r,
                "XOR" => l ^ r,
                _ => panic!(),
            };
            regs.insert(s, res);
        } else {
            work.push_back(a);
        }
    }

    let mut part1 = 0;
    for i in 0.. {
        let Some(b) = regs.get(&format!("z{:02}", i)) else {
            break;
        };
        part1 += b << i;
    }

    println!("{}", part1);

    let ops: Vec<&[String]> = input[1].chunks_exact(4).collect();

    let (mut x, mut y, mut z) = (0, 0, 0);

    for i in 0.. {
        let d = format!("{:02}", i);
        let (sx, sy, sz) = (format!("x{d}"), format!("y{d}"), format!("z{d}"));
        let (Some(xi), Some(yi), Some(zi)) = (regs.get(&sx), regs.get(&sy), regs.get(&sz)) else {
            break;
        };
        x += xi << i;
        y += yi << i;
        z += zi << i;
        if x + y & !(1 << i + 1) != z {
            let mut v = VecDeque::from_iter([&sz, &sx, &sy]);
            let mut to_print = vec![];
            while let (true, Some(s)) = (to_print.len() < 7, v.pop_front()) {
                for c in ops.iter().filter(|v| v.contains(&s)) {
                    if !to_print.contains(c) {
                        to_print.push(c);
                    }
                    [0, 2, 3].iter().for_each(|&i| {
                        v.push_back(&c[i]);
                    })
                }
            }
            println!("Error {sz}:");
            to_print.iter().for_each(|s| println!("{:?}", s));
            break;
        }
    }

    let part2 = Vec::from_iter(BTreeSet::from(["Swaps goes here..."])).join(",");

    println!("{}", part2);
}
