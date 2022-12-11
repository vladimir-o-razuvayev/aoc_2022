pub fn highest_calories(count: usize) -> usize {
    let mut calories = include_str!("day_1_input.txt")
        .lines()
        .map(|v| v.parse::<usize>().ok())
        .collect::<Vec<_>>()
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<usize>())
        .collect::<Vec<_>>();
    calories.sort_by(|a, b| b.cmp(a));
    calories.iter().take(count).sum()
}
