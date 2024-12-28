fn main() {
    let input: Vec<Vec<Vec<char>>> = aoc24::input(25)
        .split("\n\n")
        .map(|s| s.lines().map(|s| s.chars().collect()).collect())
        .collect();

    let calc = |v: &Vec<Vec<char>>| -> Vec<usize> {
        (0..5)
            .map(|c| (0..v.len() - 1).find(|&r| v[r][c] != v[r + 1][c]).unwrap())
            .collect()
    };

    let locks: Vec<Vec<_>> = input.iter().filter(|v| v[0][0] == '#').map(calc).collect();
    let keys: Vec<Vec<_>> = input.iter().filter(|v| v[0][0] == '.').map(calc).collect();

    let part1 = locks
        .iter()
        .flat_map(|l| {
            keys.iter()
                .filter(move |k| k.iter().zip(l).all(|(a, b)| 7 - a + b < 8))
        })
        .count();

    println!("{}", part1);
}
