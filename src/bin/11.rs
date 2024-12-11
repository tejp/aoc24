use std::collections::HashMap;

fn calc(stone: u64, blinks_left: u32, cache: &mut HashMap<(u64, u32), u64>) -> u64 {
    if blinks_left == 0 {
        return 1;
    }
    if let Some(&n) = cache.get(&(stone, blinks_left)) {
        return n;
    }
    let n = match stone {
        0 => calc(1, blinks_left - 1, cache),
        n if (n.ilog10() + 1) % 2 == 0 => {
            let s = format!("{n}");
            let (s1, s2) = s.split_at((n.ilog10() + 1) as usize / 2);
            let (n1, n2) = (s1.parse::<u64>().unwrap(), s2.parse::<u64>().unwrap());
            calc(n1, blinks_left - 1, cache) + calc(n2, blinks_left - 1, cache)
        }
        n => calc(n * 2024, blinks_left - 1, cache),
    };
    cache.insert((stone, blinks_left), n);
    n
}

fn main() {
    let input: Vec<u64> = aoc24::input(11)
        .split(' ')
        .map(|n| n.trim().parse().unwrap())
        .collect();

    let mut cache: HashMap<(u64, u32), u64> = HashMap::new();

    let part1: u64 = input.iter().map(|&n| calc(n, 25, &mut cache)).sum();
    println!("{}", part1);

    let part2: u64 = input.iter().map(|&n| calc(n, 75, &mut cache)).sum();
    println!("{}", part2);
}
