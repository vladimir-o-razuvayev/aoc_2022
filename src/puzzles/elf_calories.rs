use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

pub fn highest_calories() -> usize {
    let mut highest_calories = 0;
    let mut attempt = 0;
    if let Ok(lines) = read_lines("src/puzzles/elf_calories_input.txt") {
        for line in lines {
            if let Ok(text) = line {
                if text.is_empty() {
                    if attempt > highest_calories {
                        highest_calories = attempt;
                    }
                    attempt = 0;
                } else {
                    attempt += usize::from_str(&text).unwrap();
                }
            }
        }
    }
    highest_calories
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
