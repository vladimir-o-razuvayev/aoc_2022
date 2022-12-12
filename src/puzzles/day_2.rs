pub fn score() -> usize {
    include_str!("day_2_input.txt")
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            (match line.as_bytes()[2] as char {
                'X' => 0, // Lose
                'Y' => 3, // Tie
                'Z' => 6, // Win
                _ => 0,
            }) + match line {
                "B X" | "A Y" | "C Z" => 1, // Rock
                "C X" | "B Y" | "A Z" => 2, // Paper
                "A X" | "C Y" | "B Z" => 3, // Scissors
                _ => 0,
            }
        })
        .sum()
}
