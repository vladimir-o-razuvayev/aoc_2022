use crate::prelude::*;

pub fn score() -> usize {
    let mut score = 0;

    if let Ok(lines) = read_lines("src/puzzles/elf_rps_input.txt") {
        for line in lines {
            if let Ok(text) = line {
                if !text.is_empty() {
                    let l: char = text.as_bytes()[2] as char;

                    match l {
                        'X' => score += 1,
                        'Y' => score += 2,
                        'Z' => score += 3,
                        _ => println!("Invalid Shape"),
                    }

                    match text.as_str() {
                        "A X" | "B Y" | "C Z" => score += 3,
                        "A Y" | "B Z" | "C X" => score += 6,
                        "A Z" | "B X" | "C Y" => score += 0,
                        _ => println!("Invalid Input"),
                    }
                }
            }
        }
    }

    score
}
