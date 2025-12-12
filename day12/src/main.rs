use std::fs;

const PRESENT_SIZE: usize = 3;
const NUM_PRESENT_TYPES: usize = 6;

#[derive(Clone)]
struct Present([[bool; PRESENT_SIZE]; PRESENT_SIZE]);

impl From<&str> for Present {
    fn from(value: &str) -> Self {
        let mut arr = [[false; PRESENT_SIZE]; PRESENT_SIZE];

        for (r, row) in value.split("\n").enumerate() {
            for (c, ch) in row.chars().enumerate() {
                arr[r][c] = match ch {
                    '#' => true,
                    '.' => false,
                    _ => panic!("Unknown character"),
                };
            }
        }

        Self(arr)
    }
}

impl std::fmt::Debug for Present {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            self.0
                .iter()
                .map(|r| {
                    r.iter()
                        .map(|c| match *c {
                            true => "#",
                            false => ".",
                        })
                        .collect::<Vec<_>>()
                        .join("")
                })
                .collect::<Vec<_>>()
                .join("\n")
                .as_str(),
        )
    }
}

impl Present {
    fn area(&self) -> usize {
        self.0
            .iter()
            .flat_map(|r| {
                r.iter().map(|c| match *c {
                    true => 1,
                    false => 0,
                })
            })
            .sum()
    }
}

fn parse(input: &str) -> (Vec<Present>, Vec<(usize, usize, Vec<usize>)>) {
    let mut sections = input.split("\n\n");

    let presents: Vec<Present> = sections
        .by_ref()
        .take(NUM_PRESENT_TYPES)
        .map(|p| {
            let (_, grid) = p.split_once(":\n").unwrap();
            Present::from(grid)
        })
        .collect();

    let layouts: Vec<(usize, usize, Vec<usize>)> = sections
        .by_ref()
        .next()
        .unwrap()
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (dimensions, present_types) = l.split_once(": ").unwrap();
            let (r, c) = dimensions.split_once("x").unwrap();
            (
                r.parse().unwrap(),
                c.parse().unwrap(),
                present_types
                    .split(" ")
                    .map(|v| v.parse().unwrap())
                    .collect(),
            )
        })
        .collect();

    (presents, layouts)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let (presents, layouts) = parse(input.as_str());

    let est_1: usize = layouts
        .iter()
        .filter(|(r, c, v)| r * c >= PRESENT_SIZE * PRESENT_SIZE * v.iter().sum::<usize>())
        .count();

    let est_2: usize = layouts
        .iter()
        .filter(|(r, c, v)| {
            r * c
                >= presents
                    .iter()
                    .zip(v)
                    .map(|(p, i)| p.area() * i)
                    .sum::<usize>()
        })
        .count();

    println!("Pt 1 estimates: {:?}, {:?}", est_1, est_2);
}
