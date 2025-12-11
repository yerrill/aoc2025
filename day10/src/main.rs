use microlp::{LinearExpr, OptimizationDirection, Problem};
use std::{cmp::min, fs};

fn convert_line(line: &str) -> (u16, Vec<u16>, Vec<u16>) {
    let (lights, rest) = line.split_once("] ").unwrap();
    let (buttons, joltage) = rest.split_once(" {").unwrap();

    let lights: u16 = lights
        .chars()
        .filter(|ch| *ch == '#' || *ch == '.')
        .enumerate()
        .map(|(i, ch)| match ch {
            '#' => 1 << i,
            '.' => 0,
            _ => unreachable!(),
        })
        .fold(0, |acc, e| acc | e); // TRIMMING LEADING ZEROS, SHOULD BE FINE BUT WATCH

    let buttons: Vec<u16> = buttons
        .split(") (")
        .map(|s| {
            s.replace("(", "")
                .replace(")", "")
                .split(",")
                .map(|v| 1 << v.parse::<u16>().unwrap())
                .fold(0, |acc, e| acc | e)
        })
        .collect();

    let joltages: Vec<u16> = joltage
        .split(",")
        .map(|j| j.replace("{", "").replace("}", "").parse().unwrap())
        .collect();

    println!(
        "{:b} {:?} {:?}",
        lights,
        buttons
            .iter()
            .map(|b| format!("{:b}", b))
            .collect::<Vec<_>>()
            .join(","),
        joltages
            .iter()
            .map(|v| format!("{:?}", v))
            .collect::<Vec<_>>()
            .join(",")
    );

    (lights, buttons, joltages)
}

fn min_buttons((lights, buttons, _): &(u16, Vec<u16>, Vec<u16>)) -> usize {
    let mut min_count = usize::MAX;

    for n in 0..(2_u16.pow(buttons.len() as u32)) {
        let mut lights_test = 0;
        let mut count = 0;

        for (i, b) in buttons.iter().enumerate() {
            let enable = (n >> i) & 1;

            if enable > 0 {
                lights_test ^= b;
                count += 1;
            }
        }

        if lights_test == *lights {
            min_count = min(min_count, count);
        }
    }

    min_count
}

fn jolts((_, buttons, joltage): &(u16, Vec<u16>, Vec<u16>)) -> usize {
    let mut problem = Problem::new(OptimizationDirection::Minimize);
    let max = joltage.iter().copied().max().unwrap();

    let vars = (0..buttons.len())
        .map(|_| problem.add_integer_var(1.0, (0, max as i32)))
        .collect::<Vec<_>>();

    for (i, &n) in joltage.iter().enumerate() {
        problem.add_constraint(
            buttons
                .iter()
                .zip(&vars)
                .filter(|&(mask, _)| mask >> i & 1 > 0)
                .fold(LinearExpr::empty(), |mut ex, (_, &var)| {
                    ex.add(var, 1.0);
                    ex
                }),
            microlp::ComparisonOp::Eq,
            n as f64,
        );
    }

    problem.solve().unwrap().objective().round() as usize
}

fn main() {
    let input: Vec<(u16, Vec<u16>, Vec<u16>)> = fs::read_to_string("input.txt")
        .unwrap()
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(convert_line)
        .collect();

    let pt1: usize = input.iter().map(|v| min_buttons(&v)).sum();
    println!("Pt 1: {:?}", pt1);

    let pt2: usize = input.iter().map(|v| jolts(&v)).sum();
    println!("Pt 2: {:?}", pt2);
}
