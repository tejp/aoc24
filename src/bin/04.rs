fn is_xmas(t: [char; 4]) -> u64 {
    match t {
        ['X', 'M', 'A', 'S'] | ['S', 'A', 'M', 'X'] => 1,
        _ => 0,
    }
}

fn main() {
    let input: Vec<_> = aoc24::input(4)
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect();

    let mut part1 = 0;

    for i in 0..140 {
        for j in 0..140 {
            let e = i * 140 + j;
            if i < 137 {
                part1 += is_xmas([e, e + 140, e + 280, e + 420].map(|e| input[e]));
            }
            if j < 137 {
                part1 += is_xmas([e, e + 1, e + 2, e + 3].map(|e| input[e]));
            }
            if i < 137 && j < 137 {
                part1 += is_xmas([e, e + 141, e + 282, e + 423].map(|e| input[e]));
                part1 += is_xmas([e + 3, e + 142, e + 281, e + 420].map(|e| input[e]));
            }
        }
    }

    println!("{}", part1);

    let mut part2 = 0u64;
    for i in 0..138 {
        for j in 0..138 {
            let corners = String::from_iter(
                [
                    i * 140 + j,
                    i * 140 + (j + 2),
                    (i + 2) * 140 + j,
                    (i + 2) * 140 + (j + 2),
                ]
                .map(|e| input[e]),
            );
            let center = input[(i + 1) * 140 + (j + 1)];
            if center == 'A' && ["MSMS", "MMSS", "SMSM", "SSMM"].contains(&corners.as_str()) {
                part2 += 1;
            }
        }
    }

    println!("{}", part2);
}
