use std::fs;

const DIAL_MAX: i32 = 99;
const POSITIONS: i32 = DIAL_MAX + 1;

#[derive(Debug)]
enum Movement {
    Left(i32),
    Right(i32),
}

impl From<&str> for Movement {
    fn from(value: &str) -> Self {
        let mut value_iter = value.chars();

        let l_r = value_iter.next().unwrap();

        let number = value_iter.collect::<String>().parse::<i32>().unwrap();

        match l_r {
            'L' => Self::Left(number),
            'R' => Self::Right(number),
            _ => panic!("Unknown leading symbol {:?}", l_r),
        }
    }
}

#[derive(Debug)]
struct Safe {
    position: i32,
    count_landed_zero: i32,
    count_click_zero: i32,
}

impl Safe {
    fn new() -> Self {
        Self {
            position: 50,
            count_landed_zero: 0,
            count_click_zero: 0,
        }
    }

    fn make_move(self, movement: Movement) -> Self {
        let delta = match movement {
            Movement::Left(v) => -v,
            Movement::Right(v) => v,
        };

        let position = (self.position + delta).rem_euclid(POSITIONS);

        let count_landed_zero = self.count_landed_zero + if position == 0 { 1 } else { 0 };

        // Full rotations completed
        let full_rotations = delta.abs() / POSITIONS;
        // Delta without full rotations
        let normalized_delta = delta % POSITIONS;
        // Position change without full rotations
        let change = self.position + normalized_delta;

        let clicked_zero = (change > DIAL_MAX || change <= 0) && self.position != 0;

        let count_click_zero =
            self.count_click_zero + full_rotations + if clicked_zero { 1 } else { 0 };

        println!(
            "Old: {:?}, Move: {:?}, New: {:?}, count_landed_zero: {:?}, clicked_zero: {:?}, full_rotations: {:?}, count_click_zero: {:?}",
            self.position,
            delta,
            position,
            count_landed_zero,
            clicked_zero,
            full_rotations,
            count_click_zero
        );

        Self {
            position,
            count_landed_zero,
            count_click_zero,
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let parsed = input
        .split("\n")
        .filter(|s| s.len() > 0)
        .map(Movement::from)
        .fold(Safe::new(), |acc, e| acc.make_move(e));

    dbg!(parsed);
}

#[cfg(test)]
mod tests {
    use super::{Movement, Safe};

    fn assert_safe(safe: &Safe, position: i32, landed_zero: i32, click_zero: i32) {
        assert_eq!(safe.position, position, "{:?}", safe);
        assert_eq!(safe.count_landed_zero, landed_zero, "{:?}", safe);
        assert_eq!(safe.count_click_zero, click_zero, "{:?}", safe);
    }

    #[test]
    fn single() {
        let mut safe = Safe::new();

        safe = safe.make_move(Movement::Right(45));
        assert_safe(&safe, 95, 0, 0);

        safe = safe.make_move(Movement::Right(10));
        assert_safe(&safe, 5, 0, 1);

        safe = safe.make_move(Movement::Left(10));
        assert_safe(&safe, 95, 0, 2);

        safe = safe.make_move(Movement::Right(5));
        assert_safe(&safe, 0, 1, 3);

        safe = safe.make_move(Movement::Right(5));
        assert_safe(&safe, 5, 1, 3);

        safe = safe.make_move(Movement::Left(5));
        assert_safe(&safe, 0, 2, 4);

        safe = safe.make_move(Movement::Left(5));
        assert_safe(&safe, 95, 2, 4);
    }

    #[test]
    fn multiple() {
        let mut safe = Safe::new();

        safe = safe.make_move(Movement::Right(45));
        assert_safe(&safe, 95, 0, 0);

        safe = safe.make_move(Movement::Right(110));
        assert_safe(&safe, 5, 0, 2);

        safe = safe.make_move(Movement::Left(110));
        assert_safe(&safe, 95, 0, 4);

        safe = safe.make_move(Movement::Right(205));
        assert_safe(&safe, 0, 1, 7);

        safe = safe.make_move(Movement::Right(205));
        assert_safe(&safe, 5, 1, 9);

        safe = safe.make_move(Movement::Left(205));
        assert_safe(&safe, 0, 2, 12);

        safe = safe.make_move(Movement::Left(205));
        assert_safe(&safe, 95, 2, 14);
    }

    #[test]
    fn multiple_no_edge() {
        let mut safe = Safe::new();

        safe = safe.make_move(Movement::Right(45));
        assert_safe(&safe, 95, 0, 0);

        safe = safe.make_move(Movement::Left(100));
        assert_safe(&safe, 95, 0, 1);

        safe = safe.make_move(Movement::Right(100));
        assert_safe(&safe, 95, 0, 2);
    }
}
