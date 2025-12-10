use std::cmp::{max, min};
use std::collections::HashSet;
use std::error::Error;
use std::fmt;

const DELIM: char = ',';
pub type Int = i32;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, PartialOrd, Ord)]
pub struct Coord {
    pub x: Int,
    pub y: Int,
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Coord {
    pub fn from_str(input: &str) -> Result<Self, Box<dyn Error>> {
        let split = input.splitn(3, DELIM);
        let mut vals = split.map(|s| s.parse::<Int>().unwrap_or(0));

        Ok(Self {
            x: vals.next().ok_or("missing x value")?,
            y: vals.next().ok_or("missing y value")?,
        })
    }

    // Add one because the same x or y coord is treated as unit distance
    pub fn area(&self, other: &Self) -> Int {
        ((self.y - other.y).abs() + 1) * ((self.x - other.x).abs() + 1)
    }

    pub fn distance(&self, other: &Self) -> Int {
        (self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y)
    }

    pub fn interpolate(&self, other: &Self) -> Vec<Coord> {
        // Problem is constrained to either vertical or horizontal lines
        if self.y == other.y {
            let start_x = min(self.x, other.x) + 1;
            let end_x = max(self.x, other.x);

            (start_x..end_x).map(|x| Coord { x, y: self.y }).collect()
        } else {
            let start_y = min(self.y, other.y) + 1;
            let end_y = max(self.y, other.y);

            (start_y..end_y)
                .map(|y| Coord { x: self.x, y: y })
                .collect()
        }
    }

    pub fn rectangle(&self, other: &Self) -> HashSet<Coord> {
        let mut points: HashSet<Coord> = HashSet::new();

        let (min_x, max_x) = (min(self.x, other.x), max(self.x, other.x));
        let (min_y, max_y) = (min(self.y, other.y), max(self.y, other.y));

        let corners = [
            Coord { x: min_x, y: min_y }, // BL (0)
            Coord { x: max_x, y: min_y }, // BR (1)
            Coord { x: max_x, y: max_y }, // TR (2)
            Coord { x: min_x, y: max_y }, // TL (3)
        ];

        let interp: [(usize, usize); 4] = [(0, 1), (1, 2), (2, 3), (3, 1)];

        for &(i, j) in interp.iter() {
            let a = &corners[i];
            let b = &corners[j];

            let v = Self::interpolate(a, b);
            for p in v {
                points.insert(p);
            }
        }

        points
    }
}
