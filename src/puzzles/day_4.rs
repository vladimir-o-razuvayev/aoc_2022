use regex::Regex;

pub fn overlap_sum() -> usize {
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();

    include_str!("day_4_input.txt")
        .lines()
        .map(|line| {
            re.captures_iter(line)
                .map(|cap| {
                    let lx = &cap[1].parse::<usize>().unwrap();
                    let ux = &cap[2].parse::<usize>().unwrap();
                    let ly = &cap[3].parse::<usize>().unwrap();
                    let uy = &cap[4].parse::<usize>().unwrap();
                    if (uy >= ux && ly <= lx) || (uy <= ux && ly >= lx) {
                        1
                    } else {
                        0
                    }
                })
                .next()
                .unwrap()
        })
        .sum()
}

pub fn overlap_any_sum() -> usize {
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();

    include_str!("day_4_input.txt")
        .lines()
        .map(|line| {
            re.captures_iter(line)
                .map(|cap| {
                    let lx = &cap[1].parse::<usize>().unwrap();
                    let ux = &cap[2].parse::<usize>().unwrap();
                    let ly = &cap[3].parse::<usize>().unwrap();
                    let uy = &cap[4].parse::<usize>().unwrap();
                    if (uy >= ux && ly <= lx)
                        || (uy <= ux && ly >= lx)
                        || (uy >= ux && ly <= ux)
                        || (uy >= lx && ly <= lx)
                    {
                        1
                    } else {
                        0
                    }
                })
                .next()
                .unwrap()
        })
        .sum()
}
