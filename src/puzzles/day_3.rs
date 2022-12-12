use std::collections::HashSet;

pub fn sum() -> usize {
    include_str!("day_3_input.txt")
        .lines()
        .filter(|l| !l.is_empty())
        .fold(0, |mut sum, line| {
            let (head, tail) = line.split_at(line.len() / 2);
            let head: HashSet<u8> = head.bytes().into_iter().collect();
            let tail: HashSet<u8> = tail.bytes().into_iter().collect();
            for x in head.intersection(&tail) {
                if x.is_ascii_uppercase() {
                    sum += *x as usize - 38;
                } else {
                    sum += *x as usize - 96;
                }
            }
            sum
        })
}

pub fn badge_sum() -> usize {
    include_str!("day_3_input.txt")
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .fold(0, |mut sum, group_members| {
            let first: HashSet<u8> = group_members[0].bytes().into_iter().collect();
            let second: HashSet<u8> = group_members[1].bytes().into_iter().collect();
            let third: HashSet<u8> = group_members[2].bytes().into_iter().collect();
            for x in first.intersection(&second) {
                if third.contains(x) {
                    if x.is_ascii_uppercase() {
                        sum += *x as usize - 38;
                    } else {
                        sum += *x as usize - 96;
                    }
                    break;
                }
            }
            sum
        })
}
