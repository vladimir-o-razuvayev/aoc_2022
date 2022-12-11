use std::str::FromStr;

pub fn highest_calories(count: usize) -> usize {
    let mut attempt = 0;
    let mut elves: Vec<usize> = Vec::new();
    let lines = include_str!("day_1_input.txt").lines();

    for line in lines {
        if line.is_empty() {
            elves.push(attempt);
            attempt = 0;
        } else {
            attempt += usize::from_str(line).unwrap();
        }
    }

    elves.sort_by(|a, b| b.cmp(a));

    elves.iter().take(count).sum()
}
