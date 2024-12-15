use std::collections::HashSet;

struct Board {
    pos: (usize, usize),
    map: Vec<Vec<char>>,
}

impl Board {
    fn new(map: Vec<Vec<char>>) -> Self {
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if map[y][x] == '@' {
                    return Board { pos: (y, x), map };
                }
            }
        }
        panic!();
    }

    fn push(&mut self, dy: isize, dx: isize) {
        let mut to_move: Vec<(usize, usize)> = vec![];
        let mut to_check: Vec<(usize, usize)> = vec![self.pos];
        while !to_check.is_empty() {
            let mut new_to_check = vec![];
            for (y, x) in to_check {
                let c = self.map[y][x];
                if c == '#' {
                    return;
                } else if c == '@' || c == 'O' || c == '[' || c == ']' {
                    let newy = y.checked_add_signed(dy).unwrap();
                    let newx = x.checked_add_signed(dx).unwrap();
                    new_to_check.push((newy, newx));
                    to_move.push((y, x));
                    if dy != 0 {
                        if c == '[' {
                            to_move.push((y, x + 1));
                            new_to_check.push((newy, newx + 1));
                        } else if c == ']' {
                            to_move.push((y, x - 1));
                            new_to_check.push((newy, newx - 1));
                        }
                    }
                }
            }
            to_check = new_to_check;
        }
        let mut pushed = HashSet::new();
        for (y, x) in to_move.iter().rev() {
            if pushed.insert((y, x)) {
                let newy = y.checked_add_signed(dy).unwrap();
                let newx = x.checked_add_signed(dx).unwrap();
                self.map[newy][newx] = self.map[*y][*x];
                self.map[*y][*x] = '.';
                if self.map[newy][newx] == '@' {
                    self.pos = (newy, newx)
                }
            }
        }
    }

    fn calc_sum(&self) -> usize {
        let mut total = 0;
        for y in 0..self.map.len() {
            for (x, &c) in self.map[y].iter().enumerate() {
                if c == 'O' || c == '[' {
                    total += y * 100 + x;
                }
            }
        }
        total
    }
}

fn main() {
    let input = aoc24::input(15);

    let (board, moves) = input.split_once("\n\n").unwrap();

    let mut part1 = Board::new(board.lines().map(|s| s.chars().collect()).collect());
    let mut part2 = Board::new(
        board
            .lines()
            .map(|s| {
                s.chars()
                    .flat_map(|c| match c {
                        '#' => ['#', '#'],
                        'O' => ['[', ']'],
                        '.' => ['.', '.'],
                        '@' => ['@', '.'],
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect(),
    );
    let moves: Vec<char> = moves.lines().flat_map(|s| s.chars()).collect();

    for m in &moves {
        let (dy, dx) = match m {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            _ => panic!(),
        };
        part1.push(dy, dx);
        part2.push(dy, dx);
    }

    println!("{}", part1.calc_sum());
    println!("{}", part2.calc_sum());
}
