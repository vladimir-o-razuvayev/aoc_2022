use crate::prelude::*;
use std::collections::HashSet;

pub fn sum() -> usize {
    let mut sum = 0;

    if let Ok(lines) = read_lines("src/puzzles/elf_rucksack_input.txt") {
        for line in lines {
            if let Ok(text) = line {
                if !text.is_empty() {
                    let (head, tail) = text.split_at(text.len() / 2);
                    let head: HashSet<u8> = head.bytes().into_iter().collect();
                    let tail: HashSet<u8> = tail.bytes().into_iter().collect();
                    for x in head.intersection(&tail) {
                        if x.is_ascii_uppercase() {
                            sum += *x as usize - 38;
                        } else {
                            sum += *x as usize - 96;
                        }
                    }
                }
            }
        }
    }

    sum
}

pub fn badge_sum() -> usize {
    let mut sum = 0;
    let lines: Vec<&str> = include_str!("elf_rucksack_input.txt").lines().collect();
    let groups = lines.chunks(3);
    for group_members in groups {
        let first: HashSet<u8> = group_members[0].bytes().into_iter().collect();
        let second: HashSet<u8> = group_members[1].bytes().into_iter().collect();
        let third: HashSet<u8> = group_members[2].bytes().into_iter().collect();
        for x in first.intersection(&second) {
            if third.contains(x) {
                println!("what {:?}", x);
                if x.is_ascii_uppercase() {
                    sum += *x as i32 - 38;
                } else {
                    sum += *x as i32 - 96;
                }
                break;
            }
        }
    }
    sum as usize
}
