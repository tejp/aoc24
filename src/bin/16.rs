#[derive(Hash, Ord, PartialEq, PartialOrd, Eq, Debug)]
struct Position {
    y: usize,
    x: usize,
    dy: isize,
    dx: isize,
    score: i64,
}

impl Position {
    fn step(&mut self) {
        self.y = self.y.checked_add_signed(self.dy).unwrap();
        self.x = self.x.checked_add_signed(self.dx).unwrap();
        self.score += 1;
    }
}

fn main() {
    let mut board: Vec<Vec<(i64, i64)>> = aoc24::input(16)
        //_TEST_7036
        //_TEST_11048
        .lines()
        .map(|r| {
            r.chars()
                .map(|c| {
                    if c == '#' {
                        (i64::MIN, i64::MIN)
                    } else {
                        (i64::MAX, i64::MAX)
                    }
                })
                .collect()
        })
        .collect();

    let mut p = Position {
        y: board.len() - 2,
        x: 1,
        dy: 0,
        dx: 1,
        score: 0,
    };

    let mut turnpoints: Vec<Position> = vec![];
    let mut part1 = i64::MAX;
    loop {
        if p.y == 1 && p.x == board[1].len() - 2 {
            if p.score <= part1 {
                part1 = p.score;
                let (score_y, score_x) = &mut board[p.y][p.x];
                if p.dx != 0 {
                    *score_x = p.score;
                } else if p.dy != 0 {
                    *score_y = p.score;
                }
            }
            let Some(new_p) = turnpoints.pop() else {
                break;
            };
            p = new_p;
        }
        let (score_y, score_x) = &mut board[p.y][p.x];
        if p.dx != 0 && p.score <= *score_x {
            *score_x = p.score;
            turnpoints.extend([
                Position {
                    dy: 1,
                    dx: 0,
                    score: p.score + 1000,
                    ..p
                },
                Position {
                    dy: -1,
                    dx: 0,
                    score: p.score + 1000,
                    ..p
                },
            ])
        } else if p.dy != 0 && p.score <= *score_y {
            *score_y = p.score;
            turnpoints.extend([
                Position {
                    dx: 1,
                    dy: 0,
                    score: p.score + 1000,
                    ..p
                },
                Position {
                    dx: -1,
                    dy: 0,
                    score: p.score + 1000,
                    ..p
                },
            ])
        } else {
            let Some(new_p) = turnpoints.pop() else {
                break;
            };
            p = new_p;
            continue;
        };
        p.step();
    }

    println!("{}", part1);

    let (mut y, mut x, mut is_y) = (1, board[1].len() - 2, false);
    let mut to_check = vec![];
    let mut visited = vec![vec![0; board[0].len()]; board.len()];
    loop {
        let (score_y, score_x) = board[y][x];
        if is_y {
            if board[y + 1][x].0 == score_y - 1 {
                to_check.push((y + 1, x, true));
            }
            if board[y - 1][x].0 == score_y - 1 {
                to_check.push((y - 1, x, true));
            }
            if board[y][x + 1].1 == score_y - 1001 {
                to_check.push((y, x + 1, false));
            }
            if board[y][x - 1].1 == score_y - 1001 {
                to_check.push((y, x - 1, false));
            }
        } else {
            if board[y + 1][x].0 == score_x - 1001 {
                to_check.push((y + 1, x, true));
            }
            if board[y - 1][x].0 == score_x - 1001 {
                to_check.push((y - 1, x, true));
            }
            if board[y][x + 1].1 == score_x - 1 {
                to_check.push((y, x + 1, false));
            }
            if board[y][x - 1].1 == score_x - 1 {
                to_check.push((y, x - 1, false));
            }
        }
        visited[y][x] = 1;
        let Some(new_p) = to_check.pop() else {
            break;
        };
        (y, x, is_y) = new_p;
    }
    for y in 0..visited.len() {
        for x in 0..visited[0].len() {
            if visited[y][x] == 1 {
                print!("0");
                if board[y][x] == (i64::MIN, i64::MIN) {
                    panic!();
                }
            } else if board[y][x] == (i64::MIN, i64::MIN) {
                print!("|");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    let part2: i64 = visited.iter().flatten().sum();

    println!("{}", part2);
}

const _TEST_7036: &str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;

const _TEST_11048: &str = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;
