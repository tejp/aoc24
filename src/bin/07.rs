fn main() {
    let input: Vec<Vec<u64>> = aoc24::input(7)
        .lines()
        .map(|l| {
            l.split([' ', ':'])
                .filter(|&c| c != "")
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    let mut part1 = 0;
    for l in input.iter() {
        let test = l[0];
        let nums: bool = l[2..]
            .iter()
            .fold(vec![l[1]], |v, &n1| {
                let mut next: Vec<_> = vec![];
                for &n2 in v.iter() {
                    if n2 * n1 <= test {
                        next.push(n2 * n1);
                    }
                    if n2 + n1 <= test {
                        next.push(n2 + n1);
                    }
                    let new = format!("{}{}", n2, n1).parse().unwrap();
                    if new <= test {
                        next.push(new);
                    }
                }
                next
            })
            .iter()
            .any(|&n| n == test);
        if nums {
            part1 += test;
        }
    }
    println!("{:?}", part1);
}
