use std::fs;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Beam,
    Splitter,
    Start,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Empty => ".",
            Self::Beam => "|",
            Self::Splitter => "^",
            Self::Start => "S",
        })
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            'S' => Self::Start,
            '^' => Self::Splitter,
            _ => panic!("Invalid character {:?}", value),
        }
    }
}

#[derive(Clone)]
struct Grid(Vec<Vec<Tile>>);

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            self.0
                .iter()
                .map(|l| {
                    l.iter()
                        .map(|c| format!("{:?}", c))
                        .collect::<Vec<_>>()
                        .join("")
                })
                .collect::<Vec<_>>()
                .join("\n")
                .as_str(),
        )
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        Self(
            value
                .split("\n")
                .filter(|l| !l.is_empty())
                .map(|l| l.chars().map(|c| Tile::from(c)).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        )
    }
}

impl Grid {
    fn get(&self, row: isize, col: isize) -> Option<Tile> {
        let get_owned = |v: &Vec<Tile>, c: usize| match v.get(c) {
            Some(t) => Some(*t),
            None => None,
        };

        let row_negative = row < 0;
        let col_negative = col < 0;

        match (row_negative, col_negative) {
            (true, _) => None,
            (_, true) => None,
            _ => match self.0.get(row as usize) {
                None => None,
                Some(v) => get_owned(v, col as usize),
            },
        }
    }

    fn find_start(&self) -> Option<(usize, usize)> {
        for row in 0..self.0.len() {
            for col in 0..self.0[row].len() {
                if self.0[row][col] == Tile::Start {
                    return Some((row, col));
                }
            }
        }

        None
    }

    fn trace(mut self) -> (Self, usize) {
        let (row, col) = self.find_start().unwrap();
        let result = self.trace_from(row, col);
        (self, result)
    }

    fn trace_from(&mut self, row: usize, col: usize) -> usize {
        self.0[row][col] = Tile::Beam;

        match self.get(row as isize + 1, col as isize) {
            None => 0,
            Some(Tile::Empty) => self.trace_from(row + 1, col),
            Some(Tile::Splitter) => {
                let left = match self.get(row as isize + 1, col as isize - 1) {
                    Some(_) => self.trace_from(row + 1, col - 1),
                    None => 0,
                };

                let right = match self.get(row as isize + 1, col as isize + 1) {
                    Some(_) => self.trace_from(row + 1, col + 1),
                    None => 0,
                };
                left + right + 1
            }
            Some(_) => 0,
        }
    }

    fn unique_paths(mut self) -> (Self, usize) {
        let (row, col) = self.find_start().unwrap();

        let result =
            self.unique_paths_from(row, col, &mut vec![vec![0; self.0[0].len()]; self.0.len()]);

        (self, result)
    }

    fn unique_paths_from(&mut self, row: usize, col: usize, cache: &mut Vec<Vec<usize>>) -> usize {
        self.0[row][col] = Tile::Beam;

        let result = match self.get(row as isize + 1, col as isize) {
            None => 1,
            Some(Tile::Splitter) => {
                let cached = cache[row + 1][col];

                if cached > 0 {
                    return cached;
                }

                let left = match self.get(row as isize + 1, col as isize - 1) {
                    Some(_) => self.unique_paths_from(row + 1, col - 1, cache),
                    None => 0,
                };

                let right = match self.get(row as isize + 1, col as isize + 1) {
                    Some(_) => self.unique_paths_from(row + 1, col + 1, cache),
                    None => 0,
                };
                cache[row + 1][col] = left + right;
                left + right
            }
            Some(_) => self.unique_paths_from(row + 1, col, cache),
        };

        result
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let grid = Grid::from(input.as_str());

    println!("Pt 1: {:?}", grid.clone().trace().1);
    println!("Pt 2: {:?}", grid.unique_paths().1);
}
