use std::collections::HashMap;

fn main() {
    let input = aoc24::input(5);

    let (rules, pages) = input.split_once("\n\n").unwrap();

    let rules = rules.lines().fold(HashMap::new(), |mut hm, rule| {
        let [k, v]: [&str; 2] = rule.split("|").collect::<Vec<_>>().try_into().unwrap();
        hm.entry(k)
            .and_modify(|vec: &mut Vec<&str>| vec.push(v))
            .or_insert(vec![v]);
        hm
    });

    let part1: u32 = pages.lines().fold(0, |a, line| {
        let l: Vec<_> = line.split(",").collect();
        for i in 0..l.len() {
            let rules = rules.get(l[i]).unwrap();
            if l[i + 1..].iter().any(|that| !rules.contains(that)) {
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
            let r = rules.get(l[i]).unwrap();
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
        }
        else {
            a}
    });

    println!("{}", part2);
}
