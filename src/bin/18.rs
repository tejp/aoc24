fn main() {
    let input: Vec<[usize; 2]> = aoc24::input(18)
        .lines()
        .map(|s| {
            s.split(|c: char| !c.is_ascii_digit())
                .filter_map(|n| n.parse().ok())
                .collect::<Vec<usize>>()
                .try_into()
                .unwrap()
        })
        .collect();

    let (w, h) = (71, 71);
    let mut kb_snow = input[..1024]
        .into_iter()
        .fold(vec![vec![0; h]; w], |mut mat, [x, y]| {
            mat[*x][*y] = -1;
            mat
        });

    let finish = (70, 70);

    let mut next = vec![(0, 0)];

    let mut part1 = 0;

    'outer: for i in 0..input.len() {
        let mut new_next = vec![];
        for &(x, y) in &next {
            if (x, y) == finish {
                part1 = i;
                break 'outer;
            } else if x < 71 && y < 71 && kb_snow[x][y] == 0 {
                new_next.push((x + 1, y));
                new_next.push((x - 1, y));
                new_next.push((x, y + 1));
                new_next.push((x, y - 1));
                kb_snow[x][y] = 1;
            }
        }
        next = new_next
    }

    println!("{}", part1);

    let (mut lower, mut upper) = (0, input.len());

    while lower <= upper {
        next = vec![(0,0)];
        let middle = (lower + upper)/2;
        let mut kb_snow = input[..middle]
            .iter()
            .fold(vec![vec![0; h]; w], |mut mat, [x, y]| {
                mat[*x][*y] = -1;
                mat
            });
        'outer: while !next.is_empty() {
            let mut new_next = vec![];
            for &(x, y) in &next {
                if (x, y) == finish {
                    lower = middle + 1;
                    break 'outer;
                } else if x < 71 && y < 71 && kb_snow[x][y] == 0 {
                    new_next.push((x + 1, y));
                    new_next.push((x - 1, y));
                    new_next.push((x, y + 1));
                    new_next.push((x, y - 1));
                    kb_snow[x][y] = 1;
                }
            }
            next = new_next
        }
        if next.is_empty() {
            upper = middle - 1;
        }
    }

    /*for row in kb_snow {
        for cell in row {
            if cell == -1 {
                print!("#");
            } else if cell == 1 {
                print!(".")
            } else {
                print!(" ")
            }
        }
        println!("");
    }*/

    println!("{:?}", input[upper]);
}
