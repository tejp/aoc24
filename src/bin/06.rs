use std::collections::HashSet;

fn main() {
    let mut input: Vec<char> = aoc24::input(6).chars().filter(|&c| '\n' != c).collect();

    let startpos = input.iter().position(|&c| c == '^').unwrap();

    let mut directions = [-130, 1, 130, -1].iter().cycle();
    let mut d: isize = *directions.next().unwrap();

    let mut pos = startpos;
    let mut visited = vec![0u32; input.len()];

    visited[pos] = 1;

    loop {
        if (d == -130 && pos < 130)
            || (d == 1 && pos % 130 == 129)
            || (d == 130 && pos >= input.len() - 130)
            || (d == -1 && pos % 130 == 0)
        {
            break;
        }
        if input[pos.checked_add_signed(d).unwrap()] == '#' {
            d = *directions.next().unwrap();
        } else {
            pos = pos.checked_add_signed(d).unwrap();
            visited[pos] = 1;
        }
    }

    let part1: u32 = visited.iter().sum();

    println!("{:?}", part1);

    let mut part2 = 0;

    for i in 0..input.len() {
        if visited[i] == 1 && i != startpos && input[i] == '.' {
            pos = startpos;
            let mut directions = [-130, 1, 130, -1].iter().cycle();
            d = *directions.next().unwrap();
            let mut visited = HashSet::new();
            input[i] = '#';
            loop {
                if (d == -130 && pos < 130)
                    || (d == 1 && pos % 130 == 129)
                    || (d == 130 && pos >= input.len() - 130)
                    || (d == -1 && pos % 130 == 0)
                {
                    break;
                }
                if !visited.insert((pos, d)) {
                    part2 += 1;
                    break;
                }
                if input[pos.checked_add_signed(d).unwrap()] == '#' {
                    d = *directions.next().unwrap();
                } else {
                    pos = pos.checked_add_signed(d).unwrap();
                }
            }
            input[i] = '.';
        }
    }

    println!("{}", part2);
}
