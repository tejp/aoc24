struct Region {
    perimeters: u64,
    area: u64,
    corners: u64,
}

#[derive(Clone)]
struct Cell {
    peri_u: bool,
    peri_d: bool,
    peri_r: bool,
    peri_l: bool,
}

fn find_region(input: &Vec<Vec<char>>, visited: &mut Vec<Vec<Option<Cell>>>, y: usize, x: usize) -> Region {
    let e = input[y][x];

    let mut peri_dl = false;
    let mut peri_dr = false;
    let mut peri_ru = false;
    let mut peri_rd = false;
    let mut peri_lu = false;
    let mut peri_ld = false;
    let mut peri_ur = false;
    let mut peri_ul = false;

    let mut area = 1;
    let mut perimeters = 0;
    let mut corners = 0;

    let peri_u = y == 0 || input[y - 1][x] != e;
    let peri_d = y == input.len() -1 || input[y+1][x] != e;
    let peri_r = x == input[0].len()-1 || input[y][x+1] != e;
    let peri_l = x == 0 || input[y][x-1] != e;

    visited[y][x] = Some(Cell {peri_u, peri_d, peri_l, peri_r});

    if !peri_l {
        if visited[y][x-1].is_none() {
            let r = find_region(input, visited, y, x-1);
            area += r.area;
            perimeters += r.perimeters;
            corners += r.corners;
        }
        let c = visited[y][x-1].clone().unwrap();
        peri_lu = c.peri_u;
        peri_ld = c.peri_d;
    } else {
        perimeters += 1;
    }

    if !peri_r {
        if visited[y][x+1].is_none() {
            let r = find_region(input, visited, y, x+1);
            area += r.area;
            perimeters += r.perimeters;
            corners += r.corners;
        }
        let c = visited[y][x+1].clone().unwrap();
        peri_ru = c.peri_u;
        peri_rd = c.peri_d;
    } else {
        perimeters += 1;
    }

    if !peri_u {
        if visited[y - 1][x].is_none() {
            let r = find_region(input, visited, y - 1, x);
            area += r.area;
            perimeters += r.perimeters;
            corners += r.corners;
        }
        let c = visited[y - 1][x].clone().unwrap();
        peri_ul = c.peri_l;
        peri_ur = c.peri_r;
    } else {
        perimeters += 1;
    }

    if !peri_d {
        if visited[y+1][x].is_none() {
            let r = find_region(input, visited, y+1, x);
            area += r.area;
            perimeters += r.perimeters;
            corners += r.corners;
        }
        let c = visited[y + 1][x].clone().unwrap();
        peri_dl = c.peri_l;
        peri_dr = c.peri_r;
    } else {
        perimeters += 1;
    }

    if peri_u && peri_r {
        corners += 1;
    }
    if peri_r && peri_d {
        corners += 1;
    }
    if peri_d && peri_l {
        corners += 1;
    }
    if peri_l && peri_u {
        corners += 1;
    }

    if !peri_u && !peri_r {
        if peri_ur && peri_ru {
            corners += 1;
        }
    }
    if !peri_r && !peri_d {
        if peri_rd && peri_dr {
            corners += 1;
        }
    }
    if !peri_d && !peri_l {
        if peri_dl && peri_ld {
            corners += 1;
        }
    }
    if !peri_l && !peri_u {
        if peri_lu && peri_ul {
            corners += 1;
        }
    }
    
    Region { perimeters, area, corners }
}

fn main() {
    let input: Vec<Vec<char>> = (aoc24::input(12),
    r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#).0.trim().lines().map(|l| l.chars().collect()).collect();

    let mut visited: Vec<Vec<Option<Cell>>> = vec![vec![None; input[0].len()]; input.len()];

    let mut part1 = 0;
    let mut part2 = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if visited[y][x].is_none() {
                let r = find_region(&input, &mut visited, y, x);
                part1 += r.area * r.perimeters;
                part2 += r.area * r.corners;
            }
        }
    }

    println!("{}", part1);
    println!("{}", part2);
}
