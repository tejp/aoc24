use std::collections::HashMap;

fn main() {
    let input = aoc24::input(5);

    let (rules, pages) = input.split_once("\n\n").unwrap();

    let mut prio: HashMap<&str, Vec<_>> = HashMap::new();

    for rule in rules.lines() {
        let (k, v) = rule.split_once("|").unwrap();
        prio.entry(k).or_default().push(v);
    }

    let part1: u32 = pages.lines().fold(0, |a, line| {
        let l: Vec<_> = line.split(",").collect();
        for i in 0..l.len() {
            let prio = prio.get(l[i]).unwrap();
            if l[i + 1..].iter().any(|that| !prio.contains(that)) {
                return a;
            }
        }
        a + l[l.len() / 2].parse::<u32>().unwrap()
    });

    println!("{}", part1);

    let part2: u32 = pages.lines().fold(0, |a, line| {
        let mut l: Vec<_> = line.split(",").collect();
        let mut i = 0;
        let mut inc = false;
        while i < l.len() {
            let r = prio.get(l[i]).unwrap();
            i += 1;
            for j in i..l.len() {
                if !r.contains(&l[j]) {
                    i -= 1;
                    let tmp = l[i];
                    l[i] = l[j];
                    l[j] = tmp;
                    inc = true;
                    break;
                }
            }
        }
        if inc {
            a + l[l.len() / 2].parse::<u32>().unwrap()
        } else {
            a
        }
    });

    println!("{}", part2);
}
