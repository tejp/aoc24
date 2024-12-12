struct Region {
    perimeter: u64,
    area: u64,
}

fn find_region(input: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, y: usize, x: usize) -> Region {
    let e = input[y][x];
    visited[y][x] = true;

    let mut area = 1;
    let mut perimeter = 0;
    if x != 0 && input[y][x-1] == e {
        if !visited[y][x-1] {
            let r = find_region(input, visited, y, x-1);
            area += r.area;
            perimeter += r.perimeter;
        }
    } else {
        perimeter += 1;
    }

    if x != input[0].len()-1 && input[y][x+1] == e {
        if !visited[y][x+1] {
            let r = find_region(input, visited, y, x+1);
            area += r.area;
            perimeter += r.perimeter;
        }
    } else {
        perimeter += 1;
    }

    if y != 0 && input[y - 1][x] == e {
        if !visited[y - 1][x] {
            let r = find_region(input, visited, y - 1, x);
            area += r.area;
            perimeter += r.perimeter;
        }
    } else {
        perimeter += 1;
    }

    if y != input.len() -1 && input[y+1][x] == e {
        if !visited[y+1][x] {
            let r = find_region(input, visited, y+1, x);
            area += r.area;
            perimeter += r.perimeter;
        }
    } else {
        perimeter += 1;
    }
    
    Region { perimeter, area }
}

fn main() {
    let input: Vec<Vec<char>> = aoc24::input(12).trim().lines().map(|l| l.chars().collect()).collect();

    let mut visited: Vec<Vec<bool>> = vec![vec![false; input[0].len()]; input.len()];

    let mut part1 = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if !visited[y][x] {
                let r = find_region(&input, &mut visited, y, x);
                part1 += r.area * r.perimeter;
            }
        }
    }

    println!("{}", part1);
}
