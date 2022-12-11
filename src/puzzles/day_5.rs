use std::fmt;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::digit1,
    combinator::{all_consuming, map, map_res},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    Finish, IResult,
};

pub fn print_crates() -> String {
    let mut lines = include_str!("day_5_input.txt").lines();

    let crate_lines: Vec<_> = lines
        .by_ref()
        .map_while(|line| {
            all_consuming(parse_crate_line)(line)
                .finish()
                .ok()
                .map(|(_, line)| line)
        })
        .collect();

    let mut piles = transpose_rev(crate_lines);

    assert!(lines.next().unwrap().is_empty());

    for ins in lines.map(|line| all_consuming(parse_instruction)(line).finish().unwrap().1) {
        apply_rev(&mut piles, ins);
    }

    piles
        .iter()
        .map(|pile| pile.last().unwrap().to_string())
        .collect()
}

fn transpose_rev<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .rev()
                .filter_map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

// Parse Crates
fn parse_crate_line(line: &str) -> IResult<&str, Vec<Option<&str>>> {
    separated_list1(
        tag(" "),
        alt((
            map(delimited(tag("["), take(1_usize), tag("]")), Some),
            map(tag("   "), |_| None),
        )),
    )(line)
}

// Parse Intructions
fn parse_number(i: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(i)
}

fn parse_pile_number(i: &str) -> IResult<&str, usize> {
    map(parse_number, |i| i - 1)(i)
}

struct Instruction {
    quantity: usize,
    src: usize,
    dst: usize,
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "move {} from {} to {}",
            self.quantity, self.src, self.dst
        )
    }
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            preceded(tag("move "), parse_number),
            preceded(tag(" from "), parse_pile_number),
            preceded(tag(" to "), parse_pile_number),
        )),
        |(quantity, src, dst)| Instruction { quantity, src, dst },
    )(i)
}

// Piles
fn apply(piles: &mut Vec<Vec<&str>>, ins: Instruction) {
    for _ in 0..ins.quantity {
        let el = piles[ins.src].pop().unwrap();
        piles[ins.dst].push(el);
    }
}

fn apply_rev(piles: &mut Vec<Vec<&str>>, ins: Instruction) {
    let [src, dst] = piles.get_many_mut([ins.src, ins.dst]).unwrap();
    dst.extend(src.drain((src.len() - ins.quantity)..))
}
