#[derive(PartialEq, Debug, Clone, Copy)]
enum Direction {
    U,
    R,
    D,
    L,
    A,
}

use Direction::*;

fn main() {
    let input: String = aoc24::input(21);

    let codes: Vec<Vec<u32>> = input
        .lines()
        .map(|s| s.chars().filter_map(|c| c.to_digit(16)).collect())
        .collect();

    let muls: Vec<usize> = input
        .split(|c: char| !c.is_numeric())
        .filter_map(|n| n.parse().ok())
        .collect();

    let mut part1 = 0;
    for (m, mut code) in muls.iter().zip(codes) {
        let t = numerical(&mut code);
        //println!("{:?}\n{} {m}", t, t);
        part1 += m * t;
    }

    println!("{}", part1);
}

fn numerical(code: &Vec<u32>) -> usize {
    //println!("{:?}", code);
    [10].iter()
        .chain(code)
        .zip(code)
        .map(sm1)
        .map(|s| s.iter().map(|s| directional(s, 2)).min().unwrap())
        .sum()
}

fn directional(code: &Vec<Direction>, level: u64) -> usize {
    if level == 0 {
        return code.len();
    }
    //println!("{:?}", code);
    [A].iter()
        .chain(code)
        .zip(code)
        .map(sm2)
        .map(|s| s.iter().map(|s| directional(s, level - 1)).min().unwrap())
        .sum()
}

fn sm2(s: (&Direction, &Direction)) -> Vec<Vec<Direction>> {
    match s {
        (a, b) if a == b => vec![vec![A]],

        (R, U) => vec![vec![U, L, A], vec![L, U, A]],
        (R, D) => vec![vec![L, A]],
        (R, L) => vec![vec![L, L, A]],
        (R, A) => vec![vec![U, A]],

        (U, D) => vec![vec![D, A]],
        (U, L) => vec![vec![D, L, A]],
        (U, A) => vec![vec![R, A]],
        (U, R) => vec![vec![R, D, A], vec![D, R, A]],

        (D, L) => vec![vec![L, A]],
        (D, R) => vec![vec![R, A]],
        (D, A) => vec![vec![R, U, A], vec![U, R, A]],

        (L, U) => vec![vec![R, U, A]],
        (L, D) => vec![vec![R, A]],
        (L, A) => vec![vec![R, R, U, A]],

        (A, R) => vec![vec![D, A]],
        (A, U) => vec![vec![L, A]],
        (A, D) => vec![vec![D, L, A], vec![L, D, A]],
        (A, L) => vec![vec![D, L, L, A]],

        e => panic!("{:?}", e),
    }
}

fn sm1(s: (&u32, &u32)) -> Vec<Vec<Direction>> {
    match s {
        (a, b) if a == b => vec![vec![A]],

        (0, 1) => vec![vec![U, L, A]],
        (0, 2) => vec![vec![U, A]],
        (0, 3) => vec![vec![R, U, A], vec![U, R, A]],
        (0, 4) => vec![vec![U, U, L, A]],
        (0, 5) => vec![vec![U, U, A]],
        (0, 6) => vec![vec![R, U, U, A], vec![U, U, R, A]],
        (0, 7) => vec![vec![U, U, U, L, A]],
        (0, 8) => vec![vec![U, U, U, A]],
        (0, 9) => vec![vec![R, U, U, U, A], vec![U, U, U, R, A]],
        (0, 10) => vec![vec![R, A]],

        (1, 0) => vec![vec![R, D, A]],
        (1, 2) => vec![vec![R, A]],
        (1, 3) => vec![vec![R, R, A]],
        (1, 4) => vec![vec![U, A]],
        (1, 5) => vec![vec![R, U, A], vec![U, R, A]],
        (1, 6) => vec![vec![R, R, U, A], vec![U, R, R, A]],
        (1, 7) => vec![vec![U, U, A]],
        (1, 8) => vec![vec![R, U, U, A], vec![U, U, R, A]],
        (1, 9) => vec![vec![R, R, U, U, A], vec![U, U, R, R, A]],
        (1, 10) => vec![vec![R, R, D, A]],

        (2, 0) => vec![vec![D, A]],
        (2, 1) => vec![vec![L, A]],
        (2, 3) => vec![vec![R, A]],
        (2, 4) => vec![vec![U, L, A], vec![L, U, A]],
        (2, 5) => vec![vec![U, A]],
        (2, 6) => vec![vec![R, U, A], vec![U, R, A]],
        (2, 7) => vec![vec![U, U, L, A], vec![L, U, U, A]],
        (2, 8) => vec![vec![U, U, A]],
        (2, 9) => vec![vec![R, U, U, A], vec![U, U, R, A]],
        (2, 10) => vec![vec![R, D, A], vec![D, R, A]],

        (3, 0) => vec![vec![D, L, A], vec![L, D, A]],
        (3, 1) => vec![vec![L, L, A]],
        (3, 2) => vec![vec![L, A]],
        (3, 4) => vec![vec![U, L, L, A], vec![L, L, U, A]],
        (3, 5) => vec![vec![U, L, A], vec![L, U, A]],
        (3, 6) => vec![vec![U, A]],
        (3, 7) => vec![vec![U, U, L, L, A], vec![L, L, U, U, A]],
        (3, 8) => vec![vec![U, U, L, A], vec![L, U, U, A]],
        (3, 9) => vec![vec![U, U, A]],
        (3, 10) => vec![vec![D, A]],

        (4, 0) => vec![vec![R, D, D, A]],
        (4, 1) => vec![vec![D, A]],
        (4, 2) => vec![vec![R, D, A], vec![D, R, A]],
        (4, 3) => vec![vec![R, R, D, A], vec![D, R, R, A]],
        (4, 5) => vec![vec![R, A]],
        (4, 6) => vec![vec![R, R, A]],
        (4, 7) => vec![vec![U, A]],
        (4, 8) => vec![vec![R, U, A], vec![U, R, A]],
        (4, 9) => vec![vec![R, R, U, A], vec![U, R, R, A]],
        (4, 10) => vec![vec![R, R, D, D, A]],

        (5, 0) => vec![vec![D, D, A]],
        (5, 1) => vec![vec![D, L, A], vec![L, D, A]],
        (5, 2) => vec![vec![D, A]],
        (5, 3) => vec![vec![R, D, A], vec![D, R, A]],
        (5, 4) => vec![vec![L, A]],
        (5, 6) => vec![vec![R, A]],
        (5, 7) => vec![vec![U, L, A], vec![L, U, A]],
        (5, 8) => vec![vec![U, A]],
        (5, 9) => vec![vec![R, U, A], vec![U, R, A]],
        (5, 10) => vec![vec![R, D, D, A], vec![D, D, R, A]],

        (6, 0) => vec![vec![D, D, L, A], vec![L, D, D, A]],
        (6, 1) => vec![vec![D, L, L, A], vec![L, L, D, A]],
        (6, 2) => vec![vec![D, L, A], vec![L, D, A]],
        (6, 3) => vec![vec![D, A]],
        (6, 4) => vec![vec![L, L, A]],
        (6, 5) => vec![vec![L, A]],
        (6, 7) => vec![vec![U, L, L, A], vec![L, L, U, A]],
        (6, 8) => vec![vec![U, L, A], vec![L, U, A]],
        (6, 9) => vec![vec![U, A]],
        (6, 10) => vec![vec![D, D, A]],

        (7, 0) => vec![vec![R, D, D, D, A]],
        (7, 1) => vec![vec![D, D, A]],
        (7, 2) => vec![vec![R, D, D, A], vec![D, D, R, A]],
        (7, 3) => vec![vec![R, R, D, D, A], vec![D, D, R, R, A]],
        (7, 4) => vec![vec![D, A]],
        (7, 5) => vec![vec![R, D, A], vec![R, D, A]],
        (7, 6) => vec![vec![R, R, D, A], vec![D, R, R, A]],
        (7, 8) => vec![vec![R, A]],
        (7, 9) => vec![vec![R, R, A]],
        (7, 10) => vec![vec![R, R, D, D, D, A]],

        (8, 0) => vec![vec![D, D, D, A]],
        (8, 1) => vec![vec![D, D, L, A], vec![L, D, D, A]],
        (8, 2) => vec![vec![D, D, A]],
        (8, 3) => vec![vec![R, D, D, A], vec![D, D, R, A]],
        (8, 4) => vec![vec![D, L, A], vec![L, D, A]],
        (8, 5) => vec![vec![D, A]],
        (8, 6) => vec![vec![R, D, A], vec![D, R, A]],
        (8, 7) => vec![vec![L, A]],
        (8, 9) => vec![vec![R, A]],
        (8, 10) => vec![vec![R, D, D, D, A], vec![D, D, D, R, A]],

        (9, 0) => vec![vec![D, D, D, L, A], vec![L, D, D, D, A]],
        (9, 1) => vec![vec![D, D, L, L, A], vec![L, L, D, D, A]],
        (9, 2) => vec![vec![D, D, L, A], vec![L, D, D, A]],
        (9, 3) => vec![vec![D, D, A]],
        (9, 4) => vec![vec![D, L, L, A], vec![L, L, D, A]],
        (9, 5) => vec![vec![D, L, A], vec![L, D, A]],
        (9, 6) => vec![vec![D, A]],
        (9, 7) => vec![vec![L, L, A]],
        (9, 8) => vec![vec![L, A]],
        (9, 10) => vec![vec![D, D, D, A]],

        (10, 0) => vec![vec![L, A]],
        (10, 1) => vec![vec![U, L, L, A]],
        (10, 2) => vec![vec![U, L, A], vec![L, U, A]],
        (10, 3) => vec![vec![U, A]],
        (10, 4) => vec![vec![U, U, L, L, A]],
        (10, 5) => vec![vec![U, U, L, A], vec![L, U, U, A]],
        (10, 6) => vec![vec![U, U, A]],
        (10, 7) => vec![vec![U, U, U, L, L, A]],
        (10, 8) => vec![vec![U, U, U, L, A], vec![L, U, U, U, A]],
        (10, 9) => vec![vec![U, U, U, A]],

        _ => panic!(),
    }
}

const _TEST: &str = r#"029A
980A
179A
456A
379A"#;
