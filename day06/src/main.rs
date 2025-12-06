use std::collections::HashMap;
use std::fs;

// squids shouldnt do math

#[derive(Debug)]
enum Operation {
    Multiply(Vec<isize>),
    Add(Vec<isize>),
}

impl From<Vec<&str>> for Operation {
    fn from(values: Vec<&str>) -> Self {
        let mut values_iter = values.into_iter().rev();

        fn collect_values<'a>(vs: impl Iterator<Item = &'a str>) -> Vec<isize> {
            vs.map(|v| v.parse().unwrap()).collect()
        }

        match values_iter.next() {
            Some("*") => Self::Multiply(collect_values(values_iter)),
            Some("+") => Self::Add(collect_values(values_iter)),
            s => panic!("Unexpected operation {:?}", s),
        }
    }
}

impl Operation {
    fn calculate(&self) -> isize {
        match self {
            Self::Multiply(v) => v.iter().map(|v| *v).reduce(|acc, e| acc * e).unwrap(),
            Self::Add(v) => v.iter().sum(),
        }
    }
}

fn operation_with_rows(input: &str) -> isize {
    let sets = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.split(" ").filter(|ll| !ll.is_empty()).enumerate())
        .fold(HashMap::<usize, Vec<&str>>::new(), |mut acc, e| {
            e.into_iter().for_each(|(k, v)| {
                let entry = acc.entry(k).or_insert(Vec::new());
                entry.push(v);
            });
            acc
        });

    let operations = sets
        .into_iter()
        .map(|(_, v)| Operation::from(v))
        .collect::<Vec<_>>();

    operations.iter().map(|v| v.calculate()).sum::<isize>()
}

fn operation_with_cols(input: &str) -> isize {
    let grid = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().rev().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut operations = Vec::new();
    let mut values = Vec::new();
    let mut operation_sign = None;

    for col in 0..grid[0].len() {
        let mut all_spaces = true;
        let mut number_value = 0;

        for row in 0..grid.len() {
            let val = grid[row][col];

            all_spaces &= val == ' ';

            match val {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    number_value =
                        number_value * 10 + isize::try_from(val.to_digit(10).unwrap()).unwrap();
                }
                '+' => operation_sign = Some('+'),
                '*' => operation_sign = Some('*'),
                _ => {}
            };
        }
        if number_value > 0 {
            values.push(number_value);
        }

        if all_spaces {
            values = Vec::new();
            operation_sign = None;
        }

        match operation_sign {
            Some('*') => {
                operations.push(Operation::Multiply(values));
                values = Vec::new()
            }
            Some('+') => {
                operations.push(Operation::Add(values));
                values = Vec::new()
            }
            None => {}
            _ => panic!("unexpected value {:?}", operation_sign),
        };
    }

    operations.iter().map(|v| v.calculate()).sum::<isize>()
}

fn main() {
    let s_input = fs::read_to_string("sample_input.txt").unwrap();
    println!("P1 sample: {:?}", operation_with_rows(s_input.as_str()));
    println!("P2 sample: {:?}", operation_with_cols(s_input.as_str()));

    let input = fs::read_to_string("input.txt").unwrap();
    println!("P1 input: {:?}", operation_with_rows(input.as_str()));
    println!("P2 input: {:?}", operation_with_cols(input.as_str()));
}
