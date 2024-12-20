type Maze = Vec<Vec<char>>;
fn main() {
    let mut input: Maze = aoc24::input(20)
        .lines()
        .map(|s| s.chars().collect())
        .collect();

    let (h, w) = (input.len(), input[0].len());

    let (mut s, mut e) = ((0, 0), (0, 0));
    for (y, row) in input.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == 'S' {
                s = (y, x);
            } else if c == 'E' {
                e = (y, x);
            }
        }
    }

    let solve = |input: &Maze| -> usize {
        let mut visited = vec![vec![false; w]; h];
        let mut todo = vec![s];
        for i in 1.. {
            let mut new_todo = vec![];
            while let Some((y, x)) = todo.pop() {
                if (y, x) == e {
                    return i;
                }
                if !visited[y][x] && input[y][x] != '#' {
                    visited[y][x] = true;
                    new_todo.extend([(y + 1, x), (y - 1, x), (y, x + 1), (y, x - 1)]);
                }
            }
            todo = new_todo;
        }
        64
    };

    let baseline = solve(&input);

    let mut part1 = 0;

    for y in 1..h {
        for x in 1..w {
            if input[y][x] == '.' {
                if x + 2 < w && input[y][x + 1] == '#' {
                    if input[y][x + 2] == '.' {
                        input[y][x + 1] = '.';
                        if baseline - 99 > solve(&input) {
                            part1 += 1;
                        }
                        input[y][x + 1] = '#';
                    } else if x + 3 < w && input[y][x + 2] == '#' {
                        if input[y][x + 3] == '.' {
                            input[y][x + 1] = '.';
                            input[y][x + 2] = '.';
                            if baseline - 99 > solve(&input) {
                                part1 += 1;
                            }
                            input[y][x + 1] = '#';
                            input[y][x + 2] = '#';
                        }
                    }
                }
                if y + 2 < h && input[y + 1][x] == '#' {
                    if input[y + 2][x] == '.' {
                        input[y + 1][x] = '.';
                        if baseline - 99 > solve(&input) {
                            part1 += 1;
                        }
                        input[y + 1][x] = '#';
                    } else if y + 3 < h && input[y + 2][x] == '#' {
                        if input[y + 3][x] == '.' {
                            input[y + 1][x] = '.';
                            input[y + 2][x] = '.';
                            if baseline - 99 > solve(&input) {
                                part1 += 1;
                            }
                            input[y + 1][x] = '#';
                            input[y + 2][x] = '#';
                        }
                    }
                }
            }
        }
    }

    /*for row in input {
        for c in row {
            print!("{}", c);
        }
        println!();
    }*/

    println!("{}", part1);
}
