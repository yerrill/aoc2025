use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fs,
};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Point(i32, i32, i32);

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let mut p = value.split(',').map(|v| v.parse().unwrap());
        Self(p.next().unwrap(), p.next().unwrap(), p.next().unwrap())
    }
}

impl Point {
    fn to_f64(&self) -> (f64, f64, f64) {
        (f64::from(self.0), f64::from(self.1), f64::from(self.2))
    }

    fn euclid_distance(&self, other: &Self) -> f64 {
        let p1 = self.to_f64();
        let p2 = other.to_f64();

        ((p2.0 - p1.0).powi(2) + (p2.1 - p1.1).powi(2) + (p2.2 - p1.2).powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct PointPair {
    p1: Point,
    p2: Point,
    d: f64,
}

impl Ord for PointPair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.d.total_cmp(&other.d).reverse()
    }
}

impl PartialOrd for PointPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PointPair {
    fn eq(&self, other: &Self) -> bool {
        (self.p1 == other.p1 && self.p2 == other.p2) || (self.p1 == other.p2 && self.p2 == other.p1)
    }
}

impl Eq for PointPair {}

impl PointPair {
    fn new(p1: Point, p2: Point) -> Self {
        Self {
            p1,
            p2,
            d: p1.euclid_distance(&p2),
        }
    }
}

struct PointHeapIter(BinaryHeap<PointPair>);

impl Iterator for PointHeapIter {
    type Item = PointPair;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl PointHeapIter {
    fn new(points: Vec<Point>) -> Self {
        let len = points.len();

        let mut pairs = BinaryHeap::new();
        let mut seen_pairs = vec![vec![false; len]; len];

        for i in 0..points.len() {
            for j in 0..points.len() {
                if i == j || (seen_pairs[i][j] && seen_pairs[j][i]) {
                    continue;
                }

                pairs.push(PointPair::new(points[i], points[j]));

                seen_pairs[i][j] = true;
                seen_pairs[j][i] = true;
            }
        }

        Self(pairs)
    }
}

struct Group {
    heap: PointHeapIter,
    groups: HashMap<Point, u32>,
    group_id: u32,
    last_pair: Option<(i32, i32)>,
}

impl Group {
    fn new(points: Vec<Point>) -> Self {
        Self {
            heap: PointHeapIter::new(points),
            groups: HashMap::new(),
            group_id: 0,
            last_pair: None,
        }
    }

    fn advance(&mut self) -> bool {
        let Some(PointPair { p1, p2, d: _ }) = self.heap.next() else {
            return false;
        };

        let g1 = self.groups.get(&p1);
        let g2 = self.groups.get(&p2);

        let mut new_link = true;

        match (g1, g2) {
            (Some(&id1), Some(&id2)) => {
                if id1 != id2 {
                    self.groups.iter_mut().for_each(|(_, v)| {
                        if *v == id1 || *v == id2 {
                            *v = self.group_id;
                        }
                    });

                    self.group_id += 1;
                    //self.last_pair = Some((p1.0, p2.0));
                } else {
                    new_link = false;
                }
            }
            (Some(&g), None) => {
                self.groups.insert(p2, g);
                //self.last_pair = Some((p1.0, p2.0));
            }
            (None, Some(&g)) => {
                self.groups.insert(p1, g);
                //self.last_pair = Some((p1.0, p2.0));
            }
            (None, None) => {
                self.groups.insert(p1, self.group_id);
                self.groups.insert(p2, self.group_id);
                self.group_id += 1;
                //self.last_pair = Some((p1.0, p2.0));
            }
        }

        if new_link {
            self.last_pair = Some((p1.0, p2.0));
        }

        true
    }

    fn frequency(&self) -> u32 {
        let mut frequency = self
            .groups
            .values()
            .fold(HashMap::<u32, u32>::new(), |mut acc, e| {
                acc.entry(*e).and_modify(|v| *v += 1).or_insert(1);
                acc
            })
            .into_iter()
            .map(|(_, v)| v)
            .collect::<Vec<_>>();

        frequency.sort_by(|a, b| a.cmp(b).reverse());

        frequency.into_iter().take(3).product()
    }

    fn lasts(&self) -> i64 {
        match self.last_pair {
            Some((x1, x2)) => x1 as i64 * x2 as i64,
            _ => 0,
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let points = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|p| Point::from(p))
        .collect::<Vec<_>>();

    let mut group = Group::new(points);
    let mut count = 0;
    let check_at = 1000;

    while group.advance() {
        count += 1;
        if count == check_at {
            println!("Pt 1: {:?}", &group.frequency());
        }
    }

    println!("Pt 2: {:?}", &group.lasts());
}
