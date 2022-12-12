#![feature(get_many_mut)]
use puzzles::*;

mod puzzles;

fn main() {
    println!("Highest 3 Calories: {:?}", day_1::highest_calories(3));
    println!("RPS Score: {:?}", day_2::score());
    println!("Rucksack Sum: {:?}", elf_rucksack::sum());
    println!("Badge Sum: {:?}", elf_rucksack::badge_sum());
    println!("Overlap Sum: {:?}", day_4::overlap_sum());
    println!("Overlap any Sum: {:?}", day_4::overlap_any_sum());
    println!("Crates: {}", day_5::print_crates());
}

mod prelude {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}
