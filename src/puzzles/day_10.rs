#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn parse() -> Vec<Self> {
        include_str!("day_10_input.txt")
            .lines()
            .map(|line| {
                if line == "noop" {
                    Self::Noop
                } else {
                    Self::Addx(line.split_at(5).1.parse().unwrap())
                }
            })
            .collect()
    }
}

pub fn signal_sum() -> i32 {
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
        .iter()
        .enumerate()
        .map(|(i, &x)| (i as i32 + 1) * x)
        .skip(19)
        .step_by(40)
        .sum()
}
