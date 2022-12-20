use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn parse() -> Vec<Self> {
        let mut result = vec![];

        for line in include_str!("day_10_input.txt").lines() {
            if line == "noop" {
                result.push(Self::Noop);
            } else {
                result.push(Self::Addx(line.split_at(5).1.parse().unwrap()));
            }
        }
        result
    }
}

pub fn signal_sum() -> i32 {
    values()
        .iter()
        .enumerate()
        .map(|(i, &x)| (i as i32 + 1) * x)
        .skip(19)
        .step_by(40)
        .sum()
}

pub fn print_image() -> () {
    for chunk in values().iter().chunks(40).into_iter() {
        let mut line = String::new();
        for (position, value) in chunk.into_iter().enumerate() {
            if (value - position as i32).abs() <= 1 {
                line.push('#');
            } else {
                line.push(' ');
            }
        }
        println!("{}", line)
    }
}

fn values() -> Vec<i32> {
    let instructions = Instruction::parse();
    let mut x = 1;
    let mut values = Vec::new();

    for instruction in instructions {
        values.push(x);
        match instruction {
            Instruction::Noop => (),
            Instruction::Addx(i) => {
                values.push(x);
                x += i;
            }
        }
    }
    values
}
