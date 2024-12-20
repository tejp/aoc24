use std::collections::HashSet;

fn main() {
    let input: Vec<Vec<char>> = aoc24::input(20)
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

    let mut steps = vec![vec![-1; w]; h];
    let mut step_list = vec![];

    let mut todo = vec![s];
    'outer: for i in 0.. {
        let mut new_todo = vec![];
        while let Some((y, x)) = todo.pop() {
            if steps[y][x] == -1 && input[y][x] != '#' {
                steps[y][x] = i;
                step_list.push((y, x));
                if (y, x) == e {
                    break 'outer;
                }
                new_todo.extend([(y + 1, x), (y - 1, x), (y, x + 1), (y, x - 1)]);
            }
        }
        todo = new_todo;
    }

    
    let shorter_with_cheat = |cheat_length: isize| {
        let r = cheat_length + 1;
        let t: HashSet<(isize, isize)> = (0..r)
            .flat_map(|ry| (0..r - ry).map(move |rx| (ry, rx)))
            .flat_map(|(ry, rx)| [(ry, rx), (-ry, rx), (ry, -rx), (-ry, -rx)])
            .collect();
        let mut n = 0;
        for &(y, x) in &step_list {
            for &(dy, dx) in &t {
                if let (Some(y_), Some(x_)) = (y.checked_add_signed(dy), x.checked_add_signed(dx)) {
                    if x_ < w && y_ < h && steps[y][x] + 99 + dy.abs() + dx.abs() < steps[y_][x_] {
                        n += 1;
                    }
                }
            }
        }
        n
    };

    let part1 = shorter_with_cheat(2);
    let part2 = shorter_with_cheat(20);

    println!("{}", part1);
    println!("{}", part2);
}
