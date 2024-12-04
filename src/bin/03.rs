fn parse_muls(slice: &str) -> u64 {
    slice.split("mul(").skip(1).fold(0, |a, slice| {
        if let Some((l, slice)) = slice.split_once(',') {
            if let Some((r, _)) = slice.split_once(')') {
                if let (Ok(l), Ok(r)) = (l.parse::<u64>(), r.parse::<u64>()) {
                    return a + l * r;
                }
            };
        };
        a
    })
}

fn main() {
    let input = aoc24::input(3);

    let part1 = parse_muls(input.as_str());

    println!("{}", part1);

    let mut part2 = 0;

    for slice in input.split("do()") {
        let (slice, _) = slice.split_once("don't()").unwrap_or((slice, ""));

        part2 += parse_muls(slice);
    }

    println!("{}", part2);
}
