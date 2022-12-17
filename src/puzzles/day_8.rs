pub fn visible() -> usize {
    let data = parse();
    let mut count = 0;
    for (x, row) in data.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            if is_visible(&data, x, y) {
                count += count + 1;
            }
        }
    }
    count
}

pub fn max_scenic_score() -> usize {
    let data = parse();
    let mut max = 0;
    for (x, row) in data.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            let score = scenic_score(&data, x, y);
            if score > max {
                max = score;
            }
        }
    }
    max
}

fn parse() -> Vec<Vec<u32>> {
    include_str!("day_8_input.txt")
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn is_visible(data: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    if x == 0 || y == 0 || x == data.len() - 1 || y == data[x].len() - 1 {
        return true;
    }
    let l = *data[x][..y].iter().max().unwrap() < data[x][y];
    let r = *data[x][y + 1..].iter().max().unwrap() < data[x][y];
    let t = data[..x].iter().map(|v| v[y]).max().unwrap() < data[x][y];
    let b = data[x + 1..].iter().map(|v| v[y]).max().unwrap() < data[x][y];
    l || r || t || b
}

fn scenic_score(data: &Vec<Vec<u32>>, x: usize, y: usize) -> usize {
    if x == 0 || y == 0 || x == data.len() - 1 || y == data[x].len() - 1 {
        return 0;
    }
    let rc = |&v: &u32| if v < data[x][y] { Some(v) } else { None };
    let c = |v: u32| if v < data[x][y] { Some(v) } else { None };

    let len = data[x].len();
    let l = 1 + data[x][1..y].iter().rev().map_while(rc).count();
    let r = 1 + data[x][y + 1..len - 1].iter().map_while(rc).count();
    let t = 1 + data[1..x].iter().rev().map_while(|v| c(v[y])).count();
    let b = 1 + data[x + 1..len - 1].iter().map_while(|v| c(v[y])).count();
    l * r * t * b
}
