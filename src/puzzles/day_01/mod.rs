use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn highest_calories(calories: &str, count: usize) -> usize {
    let mut calories = calories
        .lines()
        .map(|v| v.parse::<usize>().ok())
        .collect::<Vec<_>>()
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<usize>())
        .collect::<Vec<_>>();
    calories.sort_by(|a, b| b.cmp(a));
    calories.iter().take(count).sum()
}
