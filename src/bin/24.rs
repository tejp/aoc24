use std::collections::HashMap;

fn main() {
    let input: Vec<Vec<String>> = aoc24::input(24)
        .split("\n\n")
        .map(|s| {
            s.split(|c: char| !c.is_alphanumeric())
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .collect();

    let mut regs: HashMap<&str, u64> = HashMap::from_iter(
        input[0]
            .iter()
            .zip(input[0][1..].iter())
            .step_by(2)
            .map(|(k, v)| (k.as_str(), v.parse().unwrap())),
    );

    let mut work = vec![];
    let mut work1 = vec![];
    let ops = input[1].iter().zip(input[1][1..].iter()).zip(input[1][2..].iter()).zip(input[1][3..].iter()).step_by(4);
    for (((l, op), r), s) in ops {
        work1.push((l, op, r, s));
    }

    while !work1.is_empty() {
        let mut new_work1 = vec![];
        for &(l, op, r, s) in &work1 {
            if let (Some(&l), Some(&r)) = (&regs.get(l.as_str()), &regs.get(r.as_str())) {
                work.push((l, op, r, s));
            } else {
                new_work1.push((l, op, r, s));
            }
        }
        work1 = new_work1;
        while let Some((l, op, r, s)) = work.pop() {
            let res = match op.as_str() {
                "AND" => l & r,
                "OR" => l | r,
                "XOR" => l ^ r,
                _ => panic!(),
            };
            regs.insert(s, res);
        }
    }

    let mut part1 = 0;
    for i in 0..99 {
        let t = format!("z{:02}", i);
        if let Some(t) = regs.get(t.as_str()) {
            part1 += t << i;
        } else {
            break;
        }
    }

    println!("{}", part1);

    let (mut x, mut y, mut z) = (0, 0, 0);

    for i in 0..99 {
        let x_ = format!("x{:02}", i);
        let y_ = format!("y{:02}", i);
        let z_ = format!("z{:02}", i);
        if regs.contains_key(x_.as_str()) {
            x += regs.get(x_.as_str()).unwrap() << i;
            y += regs.get(y_.as_str()).unwrap() << i;
        }
        if regs.contains_key(z_.as_str()) {
            z += regs.get(z_.as_str()).unwrap() << i;
        } else {
            break;
        }
    }

    println!("{:#050b}\n{:#050b}\n{:#050b}\n{:#050b}\n{:#050b}", x, y, x + y, z, (x + y) ^z);
}
