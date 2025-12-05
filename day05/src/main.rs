use std::cmp::{Ordering, max, min};
use std::collections::HashMap;
use std::fs;

type Range = (usize, usize);
type Ranges = Vec<Range>;
type Ids = Vec<usize>;

fn parse_pairs(input: &str) -> Ranges {
    input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (l, r) = l.split_once("-").unwrap();
            (l.parse().unwrap(), r.parse().unwrap())
        })
        .collect()
}

fn parse_ids(input: &str) -> Ids {
    input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}

fn parse_input(input: &str) -> (Ranges, Ids) {
    let (ranges, ids_list) = input.split_once("\n\n").unwrap();

    let range_pairs = parse_pairs(ranges);
    let ids = parse_ids(ids_list);

    (range_pairs, ids)
}

/// Compare were a number lands in relation to a range
fn num_compare(id: usize, range: &Range) -> Ordering {
    match (range.0 <= id, id <= range.1) {
        (true, true) => Ordering::Equal,
        (false, _) => Ordering::Less,
        (_, false) => Ordering::Greater,
    }
}

/// Convert set of overlapping ranges into non-overlapping ranges
fn collapse_ranges(mut ranges: Ranges) -> Ranges {
    // Sort range by minimum value ascending
    ranges.sort_by_key(|k| k.0);

    fn fold_ranges(mut acc: Ranges, e: Range) -> Ranges {
        // `num_compare` reversed because probe function tests a given range in relation to a
        // number. This is the opposite behaviour from normal.
        let index = acc.binary_search_by(|probe| num_compare(e.0, probe).reverse());

        // Modify existing range or append a non-overlapping range
        match index {
            Ok(i) => {
                let existing = acc[i];
                acc[i] = (min(existing.0, e.0), max(existing.1, e.1));
            }
            Err(i) => {
                acc.insert(i, e);
            }
        }

        acc
    }

    ranges.into_iter().fold(Vec::new(), fold_ranges)
}

/// Part 1. Check if a given ID is any range.
fn num_ids_inside_range(ranges: &Ranges, ids: Ids) -> usize {
    let mut count = 0;

    for id in ids {
        for range in ranges.iter() {
            if num_compare(id, range) == Ordering::Equal {
                count += 1;
                break;
            }
        }
    }

    count
}

/// Part 2. Directly calculate number of IDs. Requires no ranges have overlapping boundaries.
fn total_num_unique_ids(ranges: &Ranges) -> usize {
    assert!(ranges.iter().flat_map(|(l, r)| [*l, *r]).is_sorted());
    ranges.iter().map(|(l, h)| h - l + 1).sum()
}

/// Debugging. Prints ordered rank of number for easier comparison.
fn print_range(ranges: &Ranges) -> String {
    let lookup = {
        let mut v = ranges
            .iter()
            .flat_map(|(l, h)| [*l, *h])
            .collect::<Vec<_>>();
        v.sort();
        v.into_iter()
            .enumerate()
            .map(|(i, v)| (v, i))
            .collect::<HashMap<usize, usize>>()
    };

    let mut out = Vec::new();

    for (l, h) in ranges {
        out.push(format!(
            "{}({})-{}({})",
            lookup.get(l).unwrap(),
            l,
            lookup.get(h).unwrap(),
            h
        ));
    }

    out.join("\n")
}

fn main() {
    fn run(input: &str) {
        let (ranges, ids) = parse_input(input);
        println!("Original Ranges Count: {}", ranges.len());

        let new_ranges = collapse_ranges(ranges);
        println!("Non-Overlapping Ranges Count: {}", new_ranges.len());
        println!("\n{}\n", print_range(&new_ranges));

        let p1 = num_ids_inside_range(&new_ranges, ids);
        println!("P1: {}", p1);

        let p2 = total_num_unique_ids(&new_ranges);
        println!("P2: {}\n", p2);
    }
    let s_input = fs::read_to_string("sample_input.txt").unwrap();
    run(s_input.as_str());

    let input = fs::read_to_string("input.txt").unwrap();
    run(input.as_str());
}
