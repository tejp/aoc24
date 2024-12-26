use std::{collections::HashMap, iter};

use Dir::*;

#[derive(PartialEq, Debug, Clone, Copy, Eq, Hash)]
enum Dir {
    U,
    R,
    D,
    L,
    A,
}

fn main() {
    let input: String = aoc24::input(21);

    let num_codes: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();

    let muls: Vec<usize> = input
        .split(|c: char| !c.is_numeric())
        .filter_map(|n| n.parse().ok())
        .collect();

    let mut part1 = 0;
    let mut part2 = 0;
    let mut cache: HashMap<(Vec<Dir>, u64), usize> = HashMap::new();
    for (m, nums) in muls.iter().zip(num_codes) {
        let mut calc = |level| -> usize {
            iter::once(&'A')
                .chain(&nums)
                .zip(&nums)
                .map(num_on_dir)
                .map(|vv| {
                    vv.into_iter()
                        .map(|v| dir(v, level, &mut cache))
                        .min()
                        .unwrap()
                })
                .sum()
        };
        part1 += m * calc(2);
        part2 += m * calc(25);
    }

    println!("{}", part1);
    println!("{}", part2);
}

fn dir(code: Vec<Dir>, level: u64, cache: &mut HashMap<(Vec<Dir>, u64), usize>) -> usize {
    if level == 0 {
        return code.len();
    }

    if let Some(&n) = cache.get(&(code.clone(), level)) {
        return n;
    }

    let n = iter::once(&A)
        .chain(&code)
        .zip(&code)
        .map(|s| {
            dir_on_dir(s)
                .into_iter()
                .map(|code| dir(code, level - 1, cache))
                .min()
                .unwrap()
        })
        .sum();
    cache.insert((code.clone(), level), n);
    n
}

fn dir_on_dir(dir_move: (&Dir, &Dir)) -> Vec<Vec<Dir>> {
    match dir_move {
        (R, D) | (D, L) | (A, U) => vec![vec![L, A]],
        (D, R) | (L, D) | (U, A) => vec![vec![R, A]],

        (R, A) => vec![vec![U, A]],
        (A, R) => vec![vec![D, A]],

        (U, L) => vec![vec![D, L, A]],
        (L, U) => vec![vec![R, U, A]],

        (L, A) => vec![vec![R, R, U, A]],
        (A, L) => vec![vec![D, L, L, A]],

        (R, U) => vec![vec![U, L, A], vec![L, U, A]],
        (U, R) => vec![vec![R, D, A], vec![D, R, A]],

        (D, A) => vec![vec![R, U, A], vec![U, R, A]],
        (A, D) => vec![vec![D, L, A], vec![L, D, A]],

        (a, b) if a == b => vec![vec![A]],
        e => panic!("{:?}", e),
    }
}

fn num_on_dir((old, new): (&char, &char)) -> Vec<Vec<Dir>> {
    let pad_pos = |n: &char| match n.to_digit(16).unwrap() as i32 {
        0 => (0, 1),
        10 => (0, 2),
        d => ((d + 2) / 3, (d - 1) % 3),
    };
    let (old_y, old_x) = pad_pos(old);
    let (new_y, new_x) = pad_pos(new);

    let dy = new_y - old_y;
    let dx = new_x - old_x;
    let y = || iter::repeat_n(if dy < 0 { D } else { U }, dy.abs() as usize);
    let x = || iter::repeat_n(if dx < 0 { L } else { R }, dx.abs() as usize);
    let y_then_x = y().chain(x()).chain([A]).collect();
    let x_then_y = x().chain(y()).chain([A]).collect();

    match (old, new) {
        ('0' | 'A', '1' | '4' | '7') => vec![y_then_x],
        ('1' | '4' | '7', '0' | 'A') => vec![x_then_y],
        (_, _) => vec![y_then_x, x_then_y],
    }
}
