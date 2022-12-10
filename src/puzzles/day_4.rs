use regex::Regex;

pub fn overlap_sum() -> usize {
    let mut sum = 0;
    let lines: Vec<&str> = include_str!("day_4_input.txt").lines().collect();
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    for line in lines {
        for cap in re.captures_iter(line) {
            let lx = &cap[1].parse::<usize>().unwrap();
            let ux = &cap[2].parse::<usize>().unwrap();
            let ly = &cap[3].parse::<usize>().unwrap();
            let uy = &cap[4].parse::<usize>().unwrap();
            if uy >= ux && ly <= lx {
                sum += 1;
            } else if uy <= ux && ly >= lx {
                sum += 1;
            }
        }
    }
    sum
}
