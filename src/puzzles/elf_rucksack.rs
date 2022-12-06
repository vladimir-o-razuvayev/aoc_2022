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
