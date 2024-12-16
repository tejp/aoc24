#[derive(Hash, Ord, PartialEq, PartialOrd, Eq, Debug)]
struct Position {
    y: usize,
    x: usize,
    dy: isize,
    dx: isize,
    score: u64,
}

impl Position {
    fn step(&mut self) {
        self.y = self.y.checked_add_signed(self.dy).unwrap();
        self.x = self.x.checked_add_signed(self.dx).unwrap();
        self.score += 1;
    }
}

fn main() {
    let board: Vec<Vec<(u64, u64)>> = //aoc24::input(16)
        _TEST_7036.lines()
        .map(|r| r.chars().map(|c| if c == '#' {(0, 0)} else {(u64::MAX, u64::MAX)}).collect())
        .collect();

    let mut p = Position {
        y: board.len() - 2,
        x: 1,
        dy: 0,
        dx: 1,
        score: 0,
    };

    let mut turnpoints: Vec<Position> = vec![];
    let mut part1 = 0;
    loop {
        if p.y == 1 && p.x == board[1].len() - 2 {
            part1 = part1.min(p.score);
            println!("{part1}");
            let Some(p) = turnpoints.pop() else {
                break;
            };
        }
        let (mut score_y, mut score_x) = board[p.y][p.x];
        let new_p = Position { dx: 0, dy: 0, score: &p.score + 1001, ..p };
        if p.dx != 0 && p.score < score_x {
            score_x = p.score;
            turnpoints.extend([
                Position { y: p.y + 1, dy: 1, ..new_p },
                Position { y: p.y - 1, dy: -1, ..new_p },
            ])
        } else if p.dy != 0 && p.score < score_y {
            score_y = p.score;
            turnpoints.extend([
                Position {x: p.x + 1, dx: 1, ..new_p },
                Position {x: p.x - 1, dx: -1, ..new_p },
            ])
        } else {
            let Some(p) = turnpoints.pop() else {
                break;
            };
        };
        /*for (dy, dx) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            if (dy != p.dy && dx != p.dx) || (dy != -p.dy || dx != -p.dx) {
                let y = p.y.checked_add_signed(dy).unwrap();
                let x = p.x.checked_add_signed(dx).unwrap();
                if !visited[y][x] {
                    let newp = Position {
                        y,
                        x,
                        dy,
                        dx,
                        score: &p.score + 1001,
                    };
                    //println!("{:?}", newp);
                    turnpoints.push((newp, visited.clone()));
                }
            }
        }*/
        p.step();
    }

    println!("{:?}", part1);
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
