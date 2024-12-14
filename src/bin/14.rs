fn main() {
    let mut input: Vec<[i64; 4]> = aoc24::input(14)
        .lines()
        .map(|s| {
            s.split(|c: char| !c.is_ascii_digit() && c != '-')
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<i64>>()
        })
        .map(|v| v.as_slice().try_into().unwrap())
        .collect();

    let (w, h) = (101, 103);

    let mut q = [0u64; 4];
    for [x, y, vx, vy] in &input {
        let x = (x + vx * 100).rem_euclid(w);
        let y = (y + vy * 100).rem_euclid(h);

        if y < h / 2 {
            if x < w / 2 {
                q[0] += 1
            } else if x > w / 2 {
                q[1] += 1
            }
        } else if y > h / 2 {
            if x < w / 2 {
                q[2] += 1
            } else if x > w / 2 {
                q[3] += 1
            }
        }
    }

    let part1: u64 = q.iter().product();

    println!("{}", part1);

    for part2 in 1.. {
        let mut matrix = [[false; 101]; 103];
        for [x, y, vx, vy] in &mut input {
            *x = (*x + *vx).rem_euclid(w);
            *y = (*y + *vy).rem_euclid(h);

            matrix[usize::try_from(*y).unwrap()][usize::try_from(*x).unwrap()] = true;
        }

        for row in &matrix {
            if row.split(|&b| !b).any(|l| l.len() > 10) {
                return println!("{}", part2);
            }
        }
    }
}
