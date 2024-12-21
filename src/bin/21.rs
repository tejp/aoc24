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
    let input: String = _TEST.to_string();//aoc24::input(21);

    let codes: Vec<Vec<u32>> = input
        .lines()
        .map(|s| s.chars().filter_map(|c| c.to_digit(16)).collect())
        .collect();

    let muls: Vec<usize> = input.split(|c: char| !c.is_numeric()).filter_map(|n| n.parse().ok()).collect();

    let mut part1 = 0;
    for (m, mut code) in muls.iter().zip(codes) {
        let t = sm2(&mut sm2(&mut sm1(&mut code)));
        println!("{:?}\n{} {m}", t, t.len());
        part1 += m * t.len();
    }

    println!("{}", part1);
}

fn sm2(code: &mut Vec<Direction>) -> Vec<Direction> {
    code.insert(0, A);
    println!("{:?}", code);
    code.iter()
        .zip(code[1..].iter())
        .flat_map(|s| {
            // R U D L
            match s {
                (a, b) if a == b => vec![A],

                //  ^A
                // <v>

                (L, U) => vec![R, U, A],

                (R, U) => vec![U, L, A],
                (R, D) => vec![L, A],
                (R, L) => vec![L, L, A],
                (R, A) => vec![U, A],

                (U, D) => vec![D, A],
                (U, L) => vec![D, L, A],
                (U, A) => vec![R, A],

                (D, L) => vec![L, A],
                (D, A) => vec![R, U, A],

                (L, A) => vec![R, R, U, A],

                (A, R) => vec![D, A],
                (A, U) => vec![L, A],
                (A, D) => vec![D, L, A],
                (A, L) => vec![D, L, L, A],

                e => panic!("{:?}", e),
            }
        })
        .collect()
}

fn sm1(code: &mut Vec<u32>) -> Vec<Direction> {
    code.insert(0, 10);

    println!("{:?}", code);
    code.iter()
        .zip(code[1..].iter())
        .flat_map(|s| {
            match s {
                (a, b) if a == b => vec![A],

                (0, 1) => vec![U, L, A],
                (0, 2) => vec![U, A],
                (0, 3) => vec![R, U, A],
                (0, 4) => vec![U, U, L, A],
                (0, 5) => vec![U, U, A],
                (0, 6) => vec![R, U, U, A],
                (0, 7) => vec![U, U, U, L, A],
                (0, 8) => vec![U, U, U, A],
                (0, 9) => vec![R, U, U, U, A],
                (0, 10) => vec![R, A],

                (1, 0) => vec![R, D, A],
                (1, 2) => vec![R, A],
                (1, 3) => vec![R, R, A],
                (1, 4) => vec![U, A],
                (1, 5) => vec![R, U, A],
                (1, 6) => vec![R, R, U, A],
                (1, 7) => vec![U, U, A],
                (1, 8) => vec![R, U, U, A],
                (1, 9) => vec![R, R, U, U, A],
                (1, 10) => vec![R, R, D, A],

                (2, 0) => vec![D, A],
                (2, 1) => vec![L, A],
                (2, 3) => vec![R, A],
                (2, 4) => vec![U, L, A],
                (2, 5) => vec![U, A],
                (2, 6) => vec![R, U, A],
                (2, 7) => vec![U, U, L, A],
                (2, 8) => vec![U, U, A],
                (2, 9) => vec![R, U, U, A],
                (2, 10) => vec![R, D, A],

                (3, 0) => vec![D, L, A],
                (3, 1) => vec![L, L, A],
                (3, 2) => vec![L, A],
                (3, 4) => vec![U, L, L, A],
                (3, 5) => vec![U, L, A],
                (3, 6) => vec![U, A],
                (3, 7) => vec![U, U, L, L, A],
                (3, 8) => vec![U, U, L, A],
                (3, 9) => vec![U, U, A],
                (3, 10) => vec![D, A],

                (4, 0) => vec![R, D, D, A],
                (4, 1) => vec![D, A],
                (4, 2) => vec![R, D, A],
                (4, 3) => vec![R, R, D, A],
                (4, 5) => vec![R, A],
                (4, 6) => vec![R, R, A],
                (4, 7) => vec![U, A],
                (4, 8) => vec![R, U, A],
                (4, 9) => vec![R, R, U, A],
                (4, 10) => vec![R, R, D, D, A],

                (5, 0) => vec![D, D, A],
                (5, 1) => vec![D, L, A],
                (5, 2) => vec![D, A],
                (5, 3) => vec![R, D, A],
                (5, 4) => vec![L, A],
                (5, 6) => vec![R, A],
                (5, 7) => vec![U, L, A],
                (5, 8) => vec![U, A],
                (5, 9) => vec![R, U, A],
                (5, 10) => vec![R, D, D, A],

                (6, 0) => vec![D, D, L, A],
                (6, 1) => vec![D, L, L, A],
                (6, 2) => vec![D, L, A],
                (6, 3) => vec![D, A],
                (6, 4) => vec![L, L, A],
                (6, 5) => vec![L, A],
                (6, 7) => vec![U, L, L, A],
                (6, 8) => vec![U, L, A],
                (6, 9) => vec![U, A],
                (6, 10) => vec![D, D, A],

                (7, 0) => vec![R, D, D, D, A],
                (7, 1) => vec![D, D, A],
                (7, 2) => vec![R, D, D, A],
                (7, 3) => vec![R, R, D, D, A],
                (7, 4) => vec![D, A],
                (7, 5) => vec![R, D, A],
                (7, 6) => vec![R, R, D, A],
                (7, 8) => vec![R, A],
                (7, 9) => vec![R, R, A],
                (7, 10) => vec![R, R, D, D, D, A],

                (8, 0) => vec![D, D, D, A],
                (8, 1) => vec![D, D, L, A],
                (8, 2) => vec![D, D, A],
                (8, 3) => vec![R, D, D, A],
                (8, 4) => vec![D, L, A],
                (8, 5) => vec![D, A],
                (8, 6) => vec![R, D, A],
                (8, 7) => vec![L, A],
                (8, 9) => vec![R, A],
                (8, 10) => vec![R, D, D, D, A],

                (9, 0) => vec![D, D, D, L, A],
                (9, 1) => vec![D, D, L, L, A],
                (9, 2) => vec![D, D, L, A],
                (9, 3) => vec![D, D, A],
                (9, 4) => vec![D, L, L, A],
                (9, 5) => vec![D, L, A],
                (9, 6) => vec![D, A],
                (9, 7) => vec![L, L, A],
                (9, 8) => vec![L, A],
                (9, 10) => vec![D, D, D, A],

                (10, 0) => vec![L, A],
                (10, 1) => vec![U, L, L, A],
                (10, 2) => vec![U, L, A],
                (10, 3) => vec![U, A],
                (10, 4) => vec![U, U, L, L, A],
                (10, 5) => vec![U, U, L, A],
                (10, 6) => vec![U, U, A],
                (10, 7) => vec![U, U, U, L, L, A],
                (10, 8) => vec![U, U, U, L, A],
                (10, 9) => vec![U, U, U, A],

                _ => panic!(),
                /*
                        7 8 9
                        4 5 6
                        1 2 3
                          0 A
                */
            }
        })
        .collect()
}

const _TEST: &str = r#"029A
980A
179A
456A
379A"#;
