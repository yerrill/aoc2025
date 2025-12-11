use std::{collections::HashMap, fs};

const START_PT1: &str = "you";
const START_PT2: &str = "svr";
const END: &str = "out";
const STOP_1: &str = "dac";
const STOP_2: &str = "fft";

// Assumes DAG
fn dfs_dp<'a>(
    graph: &'a HashMap<String, Vec<String>>,
    from: &'a str,
    to: &str,
    paths: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(c) = paths.get(from) {
        return *c;
    }

    if from == to {
        return 1;
    }

    let Some(adjacent) = graph.get(from) else {
        return 0;
    };

    let mut count = 0;
    for adj in adjacent {
        count += dfs_dp(graph, &adj, to, paths);
    }

    paths.insert(from, count);

    count
}

fn main() {
    let input: HashMap<String, Vec<String>> = fs::read_to_string("input.txt")
        .unwrap()
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (start, connections) = l.split_once(": ").unwrap();
            (
                start.to_owned(),
                connections.split(" ").map(|s| s.to_owned()).collect(),
            )
        })
        .collect();

    let paths = |start, end| dfs_dp(&input, start, end, &mut HashMap::new());

    let pt1 = paths(START_PT1, END);

    println!("Pt 1: {:?}", pt1);

    // Given a DAG, and path must pass through 2 stops, paths must be a variation of:
    // START -> n* -> STOP 1 -> n* -> STOP 2 -> n* -> END
    // START -> n* -> STOP 2 -> n* -> STOP 1 -> n* -> END
    // where n* is any number of other nodes.
    // Number of path for either case is the product between paths(START, STOP 1) * paths(STOP 1, STOP 2) * paths(STOP 2, END)
    // Number of paths total is the sum of both cases.

    let pt2 = paths(START_PT2, STOP_1) * paths(STOP_1, STOP_2) * paths(STOP_2, END)
        + paths(START_PT2, STOP_2) * paths(STOP_2, STOP_1) * paths(STOP_1, END);

    println!("Pt 2: {:?}", pt2);
}
