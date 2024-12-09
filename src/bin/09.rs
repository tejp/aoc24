fn main() {
    let mut input: Vec<i64> = aoc24::input(9)
        //r#"2333133121414131402"#
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect();

    let mut num: usize = 0;
    /*let (mut f, mut b) = (0, input.len()>>1);
    let mut new_i = 0;
    while f <= b << 1 {
        println!("f {} new_i {} actions {}", f, new_i, input[f]);
        for _ in 0..input[f] {
            if f & 1 == 0 {
                num += new_i * f >> 1;
                println!("f add {}", new_i * f >> 1);
                new_i += 1;
            } else if b << 1 > f {

                num += new_i * b;
                println!("b add {}", new_i * b);
                input[b << 1] -= 1;
                if input[b << 1] == 0 {
                    b -= 1;
                    println!("b {}", b <<1);
                }
                new_i += 1;
            }
        }
        f += 1;
    }*/

    let mut f = 0;
    let mut new_i = 0;
    while f < input.len() {
        if f & 1 == 0 {
            if input[f] < 1 {
                new_i += input[f].abs() as usize;
                print!("{}", input[f]);
            }
            for _ in 0..input[f] {
                num += new_i * f >> 1;
                //println!("f add {}", new_i * f >> 1);
                print!("{}", f >> 1);
                new_i += 1;
            }
            f += 1;
        } else {
            let mut b = input.len() >> 1;
            while b << 1 > f && (input[b << 1] < 1 || input[f] < input[b << 1]) {
                b -= 1;
            }
            if b << 1 < f {
                for _ in 0..input[f] {
                    print!(".");
                    new_i += 1;
                }

                f += 1;
                continue;
            }
            for _ in 0..input[b << 1] {
                num += new_i * b;
                print!("{b}");
                //println!("b add {}", new_i * b);
                new_i += 1;
            }
            input[f] -= input[b << 1];
            input[b << 1] = -input[b << 1];
        }
    }

    println!("\n{}", num);
}
