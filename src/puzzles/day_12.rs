use std::fmt;
use svg::Document;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[derive(Clone, Copy)]
enum Cell {
    Start,
    End,
    Square(u8),
}

#[wasm_bindgen]
pub struct Grid {
    width: usize,
    height: usize,
    data: Vec<Vec<Cell>>,
}

#[wasm_bindgen]
impl Grid {
    #[wasm_bindgen(constructor)]
    pub fn parse(input: &str) -> Self {
        let mut data = vec![];
        for line in input.lines() {
            let mut row = vec![];
            for c in line.chars() {
                let cell = match c {
                    'S' => Cell::Start,
                    'E' => Cell::End,
                    'a'..='z' => Cell::Square(c as u8 - b'a'),
                    '\r' | '\n' => continue,
                    _ => panic!("invalid character: {c}"),
                };
                row.push(cell);
            }
            data.push(row);
        }
        let height = data.len();
        let width = data.first().unwrap().len();
        let grid = Self {
            width,
            height,
            data,
        };
        web_sys::console::log_1(&format!("{:?}", &grid).into());
        grid
    }

    fn in_bounds(&self, coord: (usize, usize)) -> bool {
        coord.0 < self.width && coord.1 < self.height
    }

    fn cell(&self, coord: (usize, usize)) -> Option<&Cell> {
        if !self.in_bounds(coord) {
            return None;
        }
        Some(&self.data[coord.1][coord.0])
    }

    #[wasm_bindgen]
    pub fn to_svg(&self) -> String {
        const SIDE: usize = 64;

        let mut document =
            Document::new().set("viewBox", (0, 0, self.width * SIDE, self.height * SIDE));

        for (y, row) in self.data.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let (title, r, g, b) = match cell {
                    Cell::Start => ("start".to_string(), 216, 27, 96),
                    Cell::End => ("end".to_string(), 30, 136, 229),
                    Cell::Square(elevation) => {
                        let f = (*elevation as f32 / 25.0 * 255.0) as u8;
                        (format!("elevation {elevation}"), f, f, f)
                    }
                };
                let rect = svg::node::element::Rectangle::new()
                    .set("x", x * SIDE)
                    .set("y", y * SIDE)
                    .set("width", SIDE)
                    .set("height", SIDE)
                    .set("fill", format!("rgb({r}, {g}, {b})"))
                    .set("stroke", "white")
                    .set("stroke-width", "2px")
                    .add(svg::node::element::Title::new().add(svg::node::Text::new(title)));
                document = document.add(rect);
            }
        }

        document.to_string()
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}x{} grid:", self.width, self.height)?;
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.cell((x, y).into()).unwrap();
                let c = match cell {
                    Cell::Start => 'S',
                    Cell::End => 'E',
                    Cell::Square(elevation) => (b'a' + elevation) as char,
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn print_grid() -> () {
    let grid = Grid::parse(include_str!("day_12_example.txt"));
    dbg!(grid);
}
