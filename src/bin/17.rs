fn main() {
    let [reg, program]: [Vec<usize>; 2] = aoc24::input(17)
        .split("\n\n")
        .map(|s| {
            s.split(|c: char| !c.is_ascii_digit())
                .filter_map(|s| s.parse().ok())
                .collect()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let mut p = 0;
    let mut output = vec![];

    let mut reg1 = reg.clone();

    while p < program.len() {

        let (inst, op) = (program[p], program[p + 1]);
        let combo = || if op > 3 { reg1[op - 4] } else { op };

        //println!("p:{p}\tinst:{}\top:{}\tco:{}\tregs:{:?}", inst, op, combo(), reg);
        match inst {
            0 => reg1[0] /= 1 << combo(),
            1 => reg1[1] ^= op,
            2 => reg1[1] = combo() % 8,
            3 => {
                if reg1[0] != 0 {
                    p = op;
                    continue;
                }
            }
            4 => reg1[1] ^= reg1[2],
            5 => output.push(combo() % 8),
            6 => reg1[1] = reg1[0] / (1 << combo()),
            7 => reg1[2] = reg1[0] / (1 << combo()),
            _ => panic!(),
        }
        p += 2;
    }

    let part1 = output
        .iter()
        .map(|d| d.to_string())
        .collect::<Vec<_>>()
        .join(",");

    println!("{}", part1);

    let mut nums = vec![0];
    let mut part2 = 0;

    for &o in program.iter().rev() {
        let mut new_nums = vec![];

        for num in &nums {
            for x in 0..8 {
                let new_num = (num << 3) + x;
                p = 0;
                output = vec![];
                reg1[0] = new_num;
                while p < program.len() - 2 {
            
                    let (inst, op) = (program[p], program[p + 1]);
                    let combo = || if op > 3 { reg1[op - 4] } else { op };
            
                    //println!("p:{p}\tinst:{}\top:{}\tco:{}\tregs:{:?}", inst, op, combo(), reg);
                    match inst {
                        0 => reg1[0] /= 1 << combo(),
                        1 => reg1[1] ^= op,
                        2 => reg1[1] = combo() % 8,
                        3 => {
                            if reg1[0] != 0 {
                                p = op;
                                continue;
                            }
                        }
                        4 => reg1[1] ^= reg1[2],
                        5 => output.push(combo() % 8),
                        6 => reg1[1] = reg1[0] / (1 << combo()),
                        7 => reg1[2] = reg1[0] / (1 << combo()),
                        _ => panic!(),
                    }
                    p += 2;
                }
                if output == vec![o] {
                    new_nums.push(new_num);
                }
            }
        }
        nums = new_nums;
        //println!("{:?}", nums);
    }
    println!("{}", nums.iter().min().unwrap());
}

const _TEST: &str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;
const _TEST2: &str = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"#;
