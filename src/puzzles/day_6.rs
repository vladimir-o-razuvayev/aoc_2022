use std::collections::HashSet;

pub fn first_marker() -> usize {
    include_str!("day_6_input.txt")
        .as_bytes()
        .windows(4)
        .position(|window| window.iter().collect::<HashSet<_>>().len() == 4)
        .map(|pos| pos + 4)
        .unwrap()
}
