use std::cmp::{max, min};
use std::collections::{HashSet, VecDeque};
use std::fs;

fn area(x1: isize, y1: isize, x2: isize, y2: isize) -> isize {
    ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1)
}

fn max_area(points: &[(isize, isize)]) -> isize {
    let mut maximum = 0;

    for (x1, y1) in points.iter() {
        for (x2, y2) in points.iter() {
            let a = area(*x1, *y1, *x2, *y2);
            maximum = max(maximum, a);
        }
    }

    maximum
}

fn bounded_max_area(points: &[(isize, isize)]) -> isize {
    // Create relative order of points
    let (xs, ys) = {
        let mut xs: Vec<isize> = points.iter().map(|(x, _)| *x).collect();
        let mut ys: Vec<isize> = points.iter().map(|(_, y)| *y).collect();

        xs.sort();
        xs.dedup();
        ys.sort();
        ys.dedup();

        (xs, ys)
    };

    let find_x = |x| (xs.iter().position(|v| *v == x).unwrap() * 2) as isize;
    let find_y = |y| (ys.iter().position(|v| *v == y).unwrap() * 2) as isize;

    let compress_x = |x1, x2| {
        let cx1 = find_x(x1);
        let cx2 = find_x(x2);

        (min(cx1, cx2), max(cx1, cx2))
    };
    let compress_y = |y1, y2| {
        let cy1 = find_y(y1);
        let cy2 = find_y(y2);

        (min(cy1, cy2), max(cy1, cy2))
    };

    // Create board (2*x - 1 to preserve gaps)
    let mut grid: Vec<Vec<isize>> = vec![vec![0; ys.len() * 2 - 1]; xs.len() * 2 - 1];

    for n in 0..points.len() {
        let (x1, y1) = points[n];
        let (x2, y2) = points[(n + 1) % points.len()];

        // Translate point to compressed version, then return min and max component
        let (cx1, cx2) = compress_x(x1, x2);
        let (cy1, cy2) = compress_y(y1, y2);

        // Fill areas of grid along the "path" (one of these loops should always be 1 iteration)
        for ix in cx1..=cx2 {
            for iy in cy1..=cy2 {
                grid[ix as usize][iy as usize] = 1;
            }
        }
    }

    let mut outside_points: HashSet<(isize, isize)> = HashSet::new();
    let mut queue: VecDeque<(isize, isize)> = VecDeque::new();

    // Point known outside of the valid range
    outside_points.insert((-1, -1));
    queue.push_back((-1, -1));

    // Floodfill the outside area
    while let Some((qx, qy)) = queue.pop_front() {
        for (nx, ny) in [(qx + 1, qy), (qx - 1, qy), (qx, qy + 1), (qx, qy - 1)] {
            // Stay in bounds of points
            if nx < -1 || ny < -1 || nx > (grid.len() as isize) || ny > (grid[0].len() as isize) {
                continue;
            }

            // If in bounds
            if nx >= 0 && nx < (grid.len() as isize) && ny >= 0 && ny < (grid[0].len() as isize) {
                // And bumping into a wall
                if grid[nx as usize][ny as usize] == 1 {
                    continue;
                }
            }

            // Already processed
            if outside_points.contains(&(nx, ny)) {
                continue;
            }

            outside_points.insert((nx, ny));
            queue.push_back((nx, ny));
        }
    }

    // Fill interior of grid
    for ix in 0..grid.len() {
        for iy in 0..grid[0].len() {
            if !outside_points.contains(&(ix as isize, iy as isize)) {
                grid[ix][iy] = 1;
            }
        }
    }

    // prefix sum array, contains total area of everything above and to the left
    let mut psa: Vec<Vec<isize>> = vec![vec![0; grid[0].len()]; grid.len()];

    for x in 0..psa.len() {
        for y in 0..psa[0].len() {
            let left = if x > 0 { psa[x - 1][y] } else { 0 };
            let top = if y > 0 { psa[x][y - 1] } else { 0 };
            let intersection = if x > 0 && y > 0 { psa[x - 1][y - 1] } else { 0 };

            psa[x][y] = left + top - intersection + grid[x][y]
        }
    }

    // check if the calculated area is the same as the PSA area (psa area has 0s when not in bounds)
    let valid_pair = |x1, y1, x2, y2| {
        let (cx1, cx2) = compress_x(x1, x2);
        let (cy1, cy2) = compress_y(y1, y2);

        // area from bottom left corner
        let left = if cx1 > 0 {
            psa[(cx1 - 1) as usize][cy2 as usize]
        } else {
            0
        };

        // area from top right corner
        let top = if cy1 > 0 {
            psa[cx2 as usize][(cy1 - 1) as usize]
        } else {
            0
        };

        // area from top left corner
        let intersection = if cx1 > 0 && cy1 > 0 {
            psa[(cx1 - 1) as usize][(cy1 - 1) as usize]
        } else {
            0
        };

        let psa_area = psa[cx2 as usize][cy2 as usize] - left - top + intersection;
        let calculated_area = (cx2 - cx1 + 1) * (cy2 - cy1 + 1);

        psa_area == calculated_area
    };

    points
        .iter()
        .flat_map(|(x1, y1)| points.iter().map(|(x2, y2)| (*x1, *y1, *x2, *y2)))
        .filter(|(x1, y1, x2, y2)| valid_pair(*x1, *y1, *x2, *y2))
        .map(|(x1, y1, x2, y2)| ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1))
        .max()
        .expect("No max found")
}

fn main() {
    let points: Vec<(isize, isize)> = fs::read_to_string("input.txt")
        .expect("Could not read input file")
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    println!("Pt 1: {:?}", max_area(&points));
    println!("Pt 2: {:?}", bounded_max_area(&points));
}
