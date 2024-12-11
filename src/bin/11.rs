fn main() {
    let binding = aoc24::input(11);
    let input: Vec<_> = binding.trim().split(' ').map(|n| n.parse::<u64>().unwrap()).collect();

    /*let part1 = input.into_iter().fold(0, |a, n| {
        let mut v = vec![n];
        for _ in 0..75 {
            let mut new_v = vec![];
            for n in v {
                match n {
                    0 => new_v.push(1),
                    n if format!("{n}").len() % 2 == 0 => {
                        let n_ = format!("{n}");
                        let (n1, n2) = n_.split_at(n_.len() / 2);
                        new_v.push(n1.parse::<u64>().unwrap());
                        new_v.push(n2.parse::<u64>().unwrap());
                    }
                    n => new_v.push(n * 2024),
                }
            }
            v = new_v;
        }
        a + v.len()
    });*/
    let part1 = (0..75).fold(input, |v: dyn Iterator, _| {
        v.flat_map(|n| {
            match n {
                0 => vec![1].iter(),
                n if format!("{n}").len() % 2 == 0 => {
                    let n_ = format!("{n}");
                    let (n1, n2) = n_.split_at(n_.len() / 2);
                    vec![n1.parse::<u64>().unwrap(), n2.parse::<u64>().unwrap()].iter()
                }
                n => {vec![n * 2024].iter()}
            }
        })
    }).count();
    
    println!("{:?}", part1);
}
