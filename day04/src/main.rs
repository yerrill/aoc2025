use std::fs;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Roll,
    RollGrabbable,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Empty => ".",
            Self::Roll => "@",
            Self::RollGrabbable => "x",
        })
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '@' => Self::Roll,
            _ => panic!("Invalid character {:?}", value),
        }
    }
}

#[derive(Clone)]
struct Warehouse(Vec<Vec<Tile>>);

impl std::fmt::Debug for Warehouse {
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

impl From<&str> for Warehouse {
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

impl Warehouse {
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

    fn find_grabbable(self) -> (Self, usize) {
        const ADJACENT: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
        ];

        let mut available_rolls = Vec::new();

        for (i, row) in self.0.iter().enumerate() {
            for (j, &tile) in row.iter().enumerate() {
                if tile == Tile::Empty {
                    continue;
                }

                let count_adjacent = ADJACENT
                    .iter()
                    .filter_map(|(x, y)| self.get(i as isize + x, j as isize + y))
                    .filter(|v| *v != Tile::Empty)
                    .count();

                if count_adjacent < 4 {
                    available_rolls.push((i, j));
                }
            }
        }

        (
            Self(
                self.0
                    .into_iter()
                    .enumerate()
                    .map(|(i, v)| {
                        v.into_iter()
                            .enumerate()
                            .map(|(j, t)| {
                                if available_rolls.contains(&(i, j)) {
                                    Tile::RollGrabbable
                                } else {
                                    t
                                }
                            })
                            .collect()
                    })
                    .collect(),
            ),
            available_rolls.len(),
        )
    }

    fn remove_grabbable(self) -> Self {
        Self(
            self.0
                .into_iter()
                .map(|v| {
                    v.into_iter()
                        .map(|t| {
                            if t == Tile::RollGrabbable {
                                Tile::Empty
                            } else {
                                t
                            }
                        })
                        .collect()
                })
                .collect(),
        )
    }

    fn remove_all(self) -> (Self, usize) {
        let (mut warehouse, mut removeable) = self.find_grabbable();
        let mut total_removed = 0;

        while removeable > 0 {
            warehouse = warehouse.remove_grabbable();
            total_removed += removeable;
            (warehouse, removeable) = warehouse.find_grabbable();
        }

        (warehouse, total_removed)
    }
}

fn main() {
    let sample_input = fs::read_to_string("sample_input.txt").unwrap();

    let sample_warehouse = Warehouse::from(sample_input.as_str());

    let (_, sample_available) = sample_warehouse.clone().find_grabbable();
    let (_, sample_removed) = sample_warehouse.remove_all();

    println!("Available (Sample): {}", sample_available);
    println!("Removed (Sample): {}", sample_removed);

    let input = fs::read_to_string("input.txt").unwrap();

    let warehouse = Warehouse::from(input.as_str());

    let (_, available) = warehouse.clone().find_grabbable();
    let (_, removed) = warehouse.remove_all();

    println!("Available (Input): {}", available);
    println!("Removed (Input): {}", removed);
}
