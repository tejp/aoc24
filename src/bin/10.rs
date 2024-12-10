use std::collections::HashMap;

fn main() {
    let input: Vec<Vec<u32>> = (
        aoc24::input(10),
        r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#,
    )
        .0
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut part1 = 0;
    let mut part2 = 0;
    for (y, v) in input.iter().enumerate() {
        for (x, _) in v.iter().enumerate().filter(|(_, &d)| d == 0) {
            let mut positions = HashMap::from([((y, x), 1u64)]);
            for d in 1..10 {
                let mut next = HashMap::new();
                for ((y, x), v) in positions {
                    if y + 1 < input.len() && input[y + 1][x] == d {
                        next.entry((y + 1, x)).and_modify(|n| *n += v).or_insert(v);
                    }
                    if y > 0 && input[y - 1][x] == d {
                        next.entry((y - 1, x)).and_modify(|n| *n += v).or_insert(v);
                    }
                    if x + 1 < input[0].len() && input[y][x + 1] == d {
                        next.entry((y, x + 1)).and_modify(|n| *n += v).or_insert(v);
                    }
                    if x > 0 && input[y][x - 1] == d {
                        next.entry((y, x - 1)).and_modify(|n| *n += v).or_insert(v);
                    }
                }
                positions = next;
            }
            part1 += positions.len();
            part2 += positions.values().sum::<u64>();
        }
    }

    println!("{}", part1);
    println!("{}", part2);
}
