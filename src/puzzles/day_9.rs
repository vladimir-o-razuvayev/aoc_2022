use itertools::Itertools;
use std::collections::HashSet;

type Point = (i32, i32);

fn lead(direction: &str, head: &mut Point) {
    match direction {
        "R" => head.0 += 1,
        "L" => head.0 -= 1,
        "D" => head.1 += 1,
        "U" => head.1 -= 1,
        _ => panic!("{direction}"),
    };
}

fn follow(leader: Point, follower: &mut Point) {
    let delta = (leader.0 - follower.0, leader.1 - follower.1);
    if delta.0.abs() == 2 || delta.1.abs() == 2 {
        follower.0 += delta.0.signum();
        follower.1 += delta.1.signum();
    }
}

pub fn visit(knot_count: usize) -> usize {
    let mut rope = vec![(0, 0); knot_count];
    let mut visited = HashSet::from([(0, 0)]);
    let motions = include_str!("day_9_input.txt")
        .split_ascii_whitespace()
        .tuples();

    for (direction, count) in motions {
        for _ in 0..count.parse().unwrap() {
            lead(direction, &mut rope[0]);
            (0..rope.len() - 1).for_each(|i| follow(rope[i], &mut rope[i + 1]));
            visited.insert(*rope.last().unwrap());
        }
    }
    visited.len()
}
