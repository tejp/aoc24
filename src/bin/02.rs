fn main() {
    let input: Vec<Vec<i64>> = aoc24::input(2)
        .lines()
        .map(|s| s.split(' ').map(|n| n.parse().unwrap()).collect())
        .collect();

    let mut part1 = 0;
    let mut unsafe_reports = vec![];
    input.iter().for_each(|e| {
        let k: Vec<i64> = e.iter().zip(e[1..].iter()).map(|(a, b)| a - b).collect();
        if k.iter().all(|&x| x > 0 && x < 4) || k.iter().all(|&x| x < 0 && x > -4) {
            part1 += 1;
        } else {
            unsafe_reports.push(e);
        }
    });

    println!("{}", part1);

    let mut part2 = part1;
    unsafe_reports.iter().for_each(|l| {
        for i in 0..l.len() {
            let l: Vec<&i64> = l[..i].iter().chain(l[i+1..].iter()).collect();
            let e: Vec<i64> = l.iter().zip(l[1..].iter()).map(|(&a, &b)| a - b).collect();
            if e.iter().all(|&x| x > 0 && x < 4) || e.iter().all(|&x| x < 0 && x > -4) {
                return part2 += 1;
            }
        }
    });

    println!("{}", part2);
}
