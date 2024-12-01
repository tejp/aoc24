fn main() {
    let input = aoc24::input(1);

    let (mut left, mut right): (Vec<u64>, Vec<u64>) = input
        .lines()
        .map(|line| {
            let mut t = line.split("   ").map(str::parse::<u64>);
            (t.next().unwrap().unwrap(), t.next().unwrap().unwrap())
        })
        .unzip();

    left.sort_unstable();
    right.sort_unstable();

    let part1: u64 = Iterator::zip(left.iter().by_ref(), right.iter().by_ref())
        .fold(0, |t, (l, r)| t + l.abs_diff(*r));

    println!("{}", part1);

    let part2: u64 = left.iter().fold(0, |t, l| {
        t + l * right.iter().filter(|&r| r == l).count() as u64
    });

    println!("{}", part2);
}
