use std::fmt;

use itertools::Itertools;

struct Monkey {
    items_held: Vec<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    divisor: usize,
    if_true: usize,
    if_false: usize,
    inspections: usize,
}

impl Monkey {
    fn parse() -> Vec<Self> {
        let mut monkies = vec![];
        for monkey in include_str!("day_11_input.txt").split("\n\n") {
            let lines: Vec<_> = monkey
                .lines()
                .map(|l| l.split(": ").last().unwrap())
                .collect();
            monkies.push(Monkey {
                items_held: lines[1].split(", ").map(|n| n.parse().unwrap()).collect(),
                operation: Self::parse_operation(lines[2]),
                divisor: lines[3].rsplit_once(' ').unwrap().1.parse().unwrap(),
                if_true: lines[4].rsplit_once(' ').unwrap().1.parse().unwrap(),
                if_false: lines[5].rsplit_once(' ').unwrap().1.parse().unwrap(),
                inspections: 0,
            })
        }
        monkies
    }

    fn parse_operation(operation: &str) -> Box<dyn Fn(usize) -> usize> {
        let op: Vec<_> = operation.rsplit_once("= ").unwrap().1.split(' ').collect();
        match op[2] {
            "old" => box |old| old * old,
            b => match (op[1], b.parse::<usize>().unwrap()) {
                ("+", n) => box move |old| old + n,
                ("*", n) => box move |old| old * n,
                _ => unreachable!(),
            },
        }
    }
}

impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Monkey:
            Starting items: {:?}
            Unique Items Inspected: {}
            Test: divisible by {}
              If true: throw to monkey {}
              If false: throw to monkey {}",
            self.items_held, self.inspections, self.divisor, self.if_true, self.if_false
        )
    }
}

pub fn print_monkies() -> usize {
    let mut monkies = Monkey::parse();
    let divisor_product: usize = monkies.iter().map(|m| m.divisor).product();
    let mut thrown_items = vec![vec![]; monkies.len()];

    for _ in 0..10000 {
        for (i, monkey) in monkies.iter_mut().enumerate() {
            monkey.items_held.append(&mut thrown_items[i]);
            for mut item in monkey.items_held.drain(0..) {
                item = (monkey.operation)(item) % divisor_product;
                thrown_items[if item % monkey.divisor == 0 {
                    monkey.if_true
                } else {
                    monkey.if_false
                }]
                .push(item);
                monkey.inspections += 1;
            }
        }
    }

    monkies
        .iter()
        .map(|monkey| monkey.inspections)
        .sorted_unstable_by(|a, b| b.cmp(a))
        .take(2)
        .product()
}
