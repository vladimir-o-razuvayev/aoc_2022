use puzzles::*;

mod puzzles;

fn main() {
    println!(
        "Highest 3 Calories: {:?}",
        elf_calories::highest_calories(3)
    );
    println!("RPS Score: {:?}", elf_rps::score());
    println!("Rucksack Sum: {:?}", elf_rucksack::sum());
    println!("Badge Sum: {:?}", elf_rucksack::badge_sum());
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
