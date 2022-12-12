pub fn score() -> usize {
    include_str!("day_2_input.txt")
        .lines()
        .filter(|l| !l.is_empty())
        .fold(0, |mut score, line| {
            let l: char = line.as_bytes()[2] as char;

            match l {
                'X' => score += 0, // Lose
                'Y' => score += 3, // Tie
                'Z' => score += 6, // Win
                _ => println!("Invalid Command"),
            };

            match line {
                "B X" | "A Y" | "C Z" => score += 1, // Rock
                "C X" | "B Y" | "A Z" => score += 2, // Paper
                "A X" | "C Y" | "B Z" => score += 3, // Scissors
                _ => println!("Invalid Input"),
            };
            score
        })
}
