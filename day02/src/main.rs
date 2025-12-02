use std::fs;

fn extract_pairs(input: String) -> Vec<(i64, i64)> {
    input
        .split(",")
        .map(|p| {
            let (v1, v2) = p.split_once("-").unwrap();
            assert!(v1.chars().next().unwrap() != '0');
            assert!(v2.chars().next().unwrap() != '0');
            (v1.parse().unwrap(), v2.parse().unwrap())
        })
        .collect()
}

/// Check if the `target` sequence repeated for the entire `input`
fn tiled_subsequence(input: &[char], target: &[char]) -> bool {
    let input_len = input.len();
    let target_len = target.len();

    if input_len == target_len {
        return false;
    }

    (0..input_len)
        .step_by(target_len)
        .all(|v| match input.get(v..(v + target_len)) {
            Some(s) => s == target,
            None => false,
        })
}

/// Run function for all values in range and return positive results
fn process_range(v1: i64, v2: i64, f: fn(i64) -> bool) -> Vec<i64> {
    (v1..(v2 + 1))
        .filter_map(|e| match f(e) {
            true => Some(i64::from(e)),
            false => None,
        })
        .collect()
}

/// Find flagged values in the list of ranges using the provided function
/// if `f` returns true, number is an invalid ID and should be flagged
fn process_pairs(values: Vec<(i64, i64)>, f: fn(i64) -> bool) -> i64 {
    values
        .into_iter()
        .flat_map(|(v1, v2)| process_range(v1, v2, f))
        .sum()
}

/// Check for any length of tiling subsequence (part 2)
fn repeating_subsequence(value: i64) -> bool {
    let chars = value.to_string().chars().collect::<Vec<_>>();

    (1..(chars.len() + 1))
        .filter_map(|v| chars.get(0..v))
        .any(|v| tiled_subsequence(chars.as_slice(), v))
}

/// Check for subsequence repeated twice (part 1)
fn half_subsequence(value: i64) -> bool {
    let chars = value.to_string().chars().collect::<Vec<_>>();

    if chars.len() % 2 != 0 {
        return false;
    }

    tiled_subsequence(chars.as_slice(), &chars[0..(chars.len() / 2)])
}

/// Run for part 1 and 2
fn run(pairs: Vec<(i64, i64)>) {
    let half = process_pairs(pairs.clone(), half_subsequence);
    println!("Repeating twice: {:?}", half);

    let full = process_pairs(pairs, repeating_subsequence);
    println!("Repeating many: {:?}", full);
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .unwrap()
        .chars()
        .filter(|&c| c != '\n')
        .collect::<String>();

    let pairs = extract_pairs(input);

    run(pairs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tiled_subs() {
        assert!(!tiled_subsequence(&['1', '1', '1'], &['1', '1', '1']));
        assert!(tiled_subsequence(&['1', '1', '1'], &['1']));
        assert!(!tiled_subsequence(&['1', '1', '1'], &['1', '1']));
        assert!(!tiled_subsequence(&['1', '2', '3'], &['1']));
        assert!(tiled_subsequence(
            &['1', '2', '3', '1', '2', '3'],
            &['1', '2', '3']
        ));
        assert!(!tiled_subsequence(
            &['1', '2', '3', '1', '2', '3', '4'],
            &['1', '2', '3']
        ));
    }

    #[test]
    fn repeating() {
        assert!(!repeating_subsequence(12345));
        assert!(repeating_subsequence(111111));
        assert!(repeating_subsequence(121212));
        assert!(repeating_subsequence(123123));
        assert!(!repeating_subsequence(1231231));
    }

    #[test]
    fn half() {
        assert!(!half_subsequence(12345));
        assert!(half_subsequence(111111));
        assert!(!half_subsequence(121212));
        assert!(half_subsequence(123123));
        assert!(!half_subsequence(1231231));
    }

    #[test]
    fn range() {
        assert!(process_range(1, 11, repeating_subsequence) == &[11]);
        assert!(process_range(11, 22, repeating_subsequence) == &[11, 22]);
        assert!(process_range(95, 115, repeating_subsequence) == &[99, 111]);
        assert!(process_range(1188511880, 1188511890, repeating_subsequence) == &[1188511885]);
        assert!(process_range(222220, 222224, repeating_subsequence) == &[222222]);
        assert!(process_range(1698522, 1698528, repeating_subsequence) == &[]);
        assert!(process_range(446443, 446449, repeating_subsequence) == &[446446]);
        assert!(process_range(38593856, 38593862, repeating_subsequence) == &[38593859]);
    }

    #[test]
    fn half_range() {
        assert!(process_range(1, 11, half_subsequence) == &[11]);
        assert!(process_range(11, 22, half_subsequence) == &[11, 22]);
        assert!(process_range(95, 115, half_subsequence) == &[99]);
        assert!(process_range(1188511880, 1188511890, half_subsequence) == &[1188511885]);
        assert!(process_range(222220, 222224, half_subsequence) == &[222222]);
        assert!(process_range(1698522, 1698528, half_subsequence) == &[]);
        assert!(process_range(446443, 446449, half_subsequence) == &[446446]);
        assert!(process_range(38593856, 38593862, half_subsequence) == &[38593859]);
    }
}
