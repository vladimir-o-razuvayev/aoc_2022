use std::collections::HashSet;

pub fn sum() -> usize {
    include_str!("day_3_input.txt")
        .lines()
        .map(|line| {
            let (head, tail) = line.split_at(line.len() / 2);
            let head: HashSet<u8> = head.bytes().into_iter().collect();
            let tail: HashSet<u8> = tail.bytes().into_iter().collect();
            head.intersection(&tail).map(|x| score(x)).next().unwrap()
        })
        .sum()
}

pub fn badge_sum() -> usize {
    include_str!("day_3_input.txt")
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group_members| {
            let first: HashSet<u8> = group_members[0].bytes().into_iter().collect();
            let second: HashSet<u8> = group_members[1].bytes().into_iter().collect();
            let third: HashSet<u8> = group_members[2].bytes().into_iter().collect();
            first
                .intersection(&second)
                .filter(|x| third.contains(x))
                .map(|x| score(x))
                .next()
                .unwrap()
        })
        .sum()
}

fn score(x: &u8) -> usize {
    if x.is_ascii_uppercase() {
        *x as usize - 38
    } else {
        *x as usize - 96
    }
}
