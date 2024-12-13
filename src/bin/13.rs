fn gcd(mut a:i64, mut b:i64) -> i64{
    if a==b { return a; }
    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }
    while b>0 {
        let temp = a;
        a = b;
        b = temp%b;
    }
    return a;
}

fn _lcm(a:i64, b:i64) -> i64{
    return a*(b/gcd(a,b));
}

fn calc_tokens(v: &Vec<i64>) -> i64 {
    let [ax, ay, bx, by, mut x, mut y] = v.as_slice().try_into().unwrap();
    x += 10000000000000;
    y += 10000000000000;
    let d1 = gcd(ax, ay);
    let d2 = gcd(bx, by);

    let b1 = (y*(ax/d1) - x*(ay/d1)) / (by*(ax/d1) - bx*(ay/d1));
    if (y*(ax/d1) - x*(ay/d1)) % (by*(ax/d1) - bx*(ay/d1)) != 0 || (y - by * b1) % ay != 0 {
        let a2 = (y*(bx/d2) - x*(by/d2)) / (ay*(bx/d2) - ax*(by/d2));

        if (y*(bx/d2) - x*(by/d2)) % (ay*(bx/d2) - ax*(by/d2)) != 0 || (y - ay * a2) % by !=0 {
            return 0
        }
        let b2 = (y - ay * a2) / by;
        return 3 * a2 + b2;
        /*if a2 > 0 && b2 > 0 && a2 < 101 && b2 < 101 {
        } else {
            return 0;
        }*/
    }

    let a1 = (y - by * b1) / ay;


    //println!("{} {}\n", gcd(ax, bx), gcd(ay, by));
    //println!("{} {} {}", a1, b1, 3 * a1 + b1);
    //println!("{} {} {}", a2, b2, 3 * a2 + b2);

    //if a1 > 0 && b1 > 0 && a1 < 101 && b1 < 101 {
        3 * a1 + b1
    /* } else {
        0
    }*/
}

fn main() {
    let input: Vec<Vec<i64>> = aoc24::input(13)
        .split("\n\n")
        .map(|s| {
            s.split(|c: char| !c.is_ascii_digit())
                .filter(|&s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    //println!("{}", calc_tokens(&vec![94,34,22,67,8400,5400]));
    //println!("{}", calc_tokens(&vec![26, 66, 67, 21, 12748, 12176]));
    //println!("{}", calc_tokens(&vec![17, 86, 84, 37, 7870, 6450]));
    let part1: i64 = input.iter().map(|v| calc_tokens(v)).sum();

    println!("{:?}", part1);
}
