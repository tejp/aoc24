use std::collections::HashMap;

fn main() {
    let input: Vec<Vec<char>> = aoc24::input(8)
        .lines()
        .map(|s| s.chars().collect())
        .collect();

    let mut hm: HashMap<char, Vec<_>> = HashMap::new();

    for (i, v) in input.iter().enumerate() {
        for (j, &c) in v.iter().enumerate() {
            if c != '.' {
                hm.entry(c).or_default().push((i as isize, j as isize));
            }
        }
    }

    let is_in = |i: isize, j: isize| {
        i >= 0 && i < input.len() as isize && j >= 0 && j < input[0].len() as isize
    };

    let mut antinodes = vec![vec![0u32; input[0].len()]; input.len()];

    for (_, v) in hm.iter() {
        for k in 0..v.len() {
            let (i1, j1) = v[k];
            for (i2, j2) in v[k + 1..].iter() {
                let (di, dj) = (i1 - i2, j1 - j2);
                let ((a1_i, a1_j), (a2_i, a2_j)) = ((i1 + di, j1 + dj), (i2 - di, j2 - dj));
                if is_in(a1_i, a1_j) {
                    antinodes[a1_i as usize][a1_j as usize] = 1;
                }
                if is_in(a2_i, a2_j) {
                    antinodes[a2_i as usize][a2_j as usize] = 1;
                }
            }
        }
    }

    println!("{}", antinodes.iter().flatten().sum::<u32>());

    for (_, v) in hm.iter() {
        for k in 0..v.len() {
            let (i1, j1) = v[k];
            for (i2, j2) in v[k + 1..].iter() {
                let (di, dj) = (i1 - i2, j1 - j2);
                let (mut a_i, mut a_j) = (i1, j1);
                while is_in(a_i, a_j) {
                    antinodes[a_i as usize][a_j as usize] = 1;
                    (a_i, a_j) = (a_i + di, a_j + dj)
                }
                (a_i, a_j) = (*i2, *j2);
                while is_in(a_i, a_j) {
                    antinodes[a_i as usize][a_j as usize] = 1;
                    (a_i, a_j) = (a_i - di, a_j - dj)
                }
            }
        }
    }

    println!("{}", antinodes.iter().flatten().sum::<u32>());
}
