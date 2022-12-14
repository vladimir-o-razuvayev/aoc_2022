use itertools::Itertools;

pub fn first_marker(length: usize) -> usize {
    include_str!("day_6_input.txt")
        .as_bytes()
        .windows(length)
        .position(|window| window.iter().unique().count() == length)
        .map(|pos| pos + length)
        .unwrap()
}
