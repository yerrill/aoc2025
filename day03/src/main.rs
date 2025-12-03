use std::cmp::{max, min};
use std::fs;

struct Cache(Vec<Vec<usize>>);

impl Cache {
    fn new(size: usize) -> Self {
        Self(vec![vec![0; size]; size]) // cache[starting_position][wanted_digits]
    }

    fn set(&mut self, starting_position: usize, wanted_count: usize, value: usize) {
        self.0[starting_position][wanted_count - 1] = value;
    }

    fn get(&self, starting_position: usize, wanted_count: usize) -> Option<usize> {
        let v = self.0[starting_position][wanted_count - 1];

        if v != 0 { Some(v) } else { None }
    }
}

impl std::fmt::Debug for Cache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            self.0
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    format!(
                        "Starting From: {} | {}",
                        i,
                        v.iter()
                            .map(|n| n.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                })
                .collect::<Vec<_>>()
                .join("\n")
                .as_str(),
        )
    }
}

#[derive(Debug)]
struct Bank(Vec<u8>);

impl From<&str> for Bank {
    fn from(value: &str) -> Self {
        Bank(
            value
                .chars()
                .map(|c| u8::try_from(c.to_digit(10).unwrap()).unwrap())
                .collect(),
        )
    }
}

impl Bank {
    fn max_of_count_batteries(&self, count: usize) -> usize {
        let mut cache = Cache::new(self.0.len());
        self.fill_cache(&mut cache, count);
        cache.get(0, count).unwrap()
    }

    fn fill_cache(&self, cache: &mut Cache, max_count: usize) {
        let mut starting_position;

        let size = self.0.len();

        for i in 1..=size {
            starting_position = size - i;

            for count in 1..=(min(i, max_count)) {
                cache.set(
                    starting_position,
                    count,
                    self.get_best(starting_position, count, cache),
                );
            }
        }
    }

    fn get_best(&self, start: usize, count: usize, cache: &Cache) -> usize {
        if start + count == self.0.len() {
            return self.0[start..]
                .iter()
                .fold(0, |acc, e| acc * 10 + *e as usize);
        }

        if count <= 1 {
            return *self.0[start..].iter().max().unwrap() as usize;
        }

        let existing_best = cache
            .get(start + 1, count)
            .expect("existing_best get should not be None");

        let new_option = (self.0[start] as usize) * 10usize.pow((count - 1) as u32)
            + cache
                .get(start + 1, count - 1)
                .expect("new_option get should not be None");

        max(existing_best, new_option)
    }
}

#[derive(Debug)]
struct Generator(Vec<Bank>);

impl From<&str> for Generator {
    fn from(value: &str) -> Self {
        Self(
            value
                .split("\n")
                .filter(|l| !l.is_empty())
                .map(|l| l.into())
                .collect(),
        )
    }
}

impl Generator {
    fn max_count_batteries(&self, count: usize) -> (Vec<usize>, usize) {
        let values = self
            .0
            .iter()
            .map(|v| v.max_of_count_batteries(count))
            .collect::<Vec<_>>();

        let sum = values.iter().sum();

        (values, sum)
    }
}

fn main() {
    let sample_input = fs::read_to_string("sample_input.txt").unwrap();

    let sample_generator = Generator::from(sample_input.as_str());

    println!(
        "Sample Part 1: {}",
        sample_generator.max_count_batteries(2).1
    );
    println!(
        "Sample Part 2: {}",
        sample_generator.max_count_batteries(12).1
    );

    let input = fs::read_to_string("input.txt").unwrap();

    let generator = Generator::from(input.as_str());

    println!("Input Part 1: {}", generator.max_count_batteries(2).1);
    println!("Input Part 2: {}", generator.max_count_batteries(12).1);
}
