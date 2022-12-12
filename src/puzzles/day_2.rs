use crate::prelude::*;

pub fn score() -> usize {
    let mut score = 0;

    if let Ok(lines) = read_lines("src/puzzles/elf_rps_input.txt") {
        for line in lines {
            if let Ok(text) = line {
                if !text.is_empty() {
                    let l: char = text.as_bytes()[2] as char;

                    match l {
                        'X' => score += 0, // Lose
                        'Y' => score += 3, // Tie
                        'Z' => score += 6, // Win
                        _ => println!("Invalid Command"),
                    }

                    match text.as_str() {
                        "B X" | "A Y" | "C Z" => score += 1, // Rock
                        "C X" | "B Y" | "A Z" => score += 2, // Paper
                        "A X" | "C Y" | "B Z" => score += 3, // Scissors
                        _ => println!("Invalid Input"),
                    }
                }
            }
        }
    }

    score
}
