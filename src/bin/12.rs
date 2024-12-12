struct Region {
    perimeters: u64,
    area: u64,
    corners: u64,
}

fn get_region(input: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, y: usize, x: usize) -> Region {
    let e = input[y][x];

    let mut area = 1;
    let mut perimeters = 0;
    let mut corners = 0;

    let peri_u = y == 0 || input[y - 1][x] != e;
    let peri_d = y == input.len() - 1 || input[y + 1][x] != e;
    let peri_r = x == input[0].len() - 1 || input[y][x + 1] != e;
    let peri_l = x == 0 || input[y][x - 1] != e;

    visited[y][x] = true;

    let mut handle = |peri: bool, (y, x): (usize, usize)| {
        if !peri {
            if !visited[y][x] {
                let r = get_region(input, visited, y, x);
                area += r.area;
                perimeters += r.perimeters;
                corners += r.corners;
            }
        } else {
            perimeters += 1;
        }
    };

    handle(peri_u, (y - 1, x));
    handle(peri_r, (y, x + 1));
    handle(peri_d, (y + 1, x));
    handle(peri_l, (y, x - 1));

    corners += [
        peri_u && peri_r,
        peri_r && peri_d,
        peri_d && peri_l,
        peri_l && peri_u,
        !peri_u && !peri_r && input[y - 1][x + 1] != e,
        !peri_r && !peri_d && input[y + 1][x + 1] != e,
        !peri_d && !peri_l && input[y + 1][x - 1] != e,
        !peri_l && !peri_u && input[y - 1][x - 1] != e,
    ]
    .iter()
    .fold(0, |a, &b| if b { a + 1 } else { a });

    Region {
        perimeters,
        area,
        corners,
    }
}

fn main() {
    let input: Vec<Vec<char>> = aoc24::input(12)
        .trim()
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let mut visited: Vec<Vec<bool>> = vec![vec![false; input[0].len()]; input.len()];

    let mut part1 = 0;
    let mut part2 = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if !visited[y][x] {
                let r = get_region(&input, &mut visited, y, x);
                part1 += r.area * r.perimeters;
                part2 += r.area * r.corners;
            }
        }
    }

    println!("{}", part1);
    println!("{}", part2);
}
