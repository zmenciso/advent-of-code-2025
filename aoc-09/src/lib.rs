use derive_more::{Add, AddAssign, Sub, SubAssign};
use std::cmp::{max, min};
use std::error::Error;
use std::fmt;

const DELIM: char = ',';
pub type Int = isize;
pub type Segment = (Coord, Coord);

#[derive(
    Debug, Hash, Eq, PartialEq, Copy, Clone, PartialOrd, Ord, Add, Sub, AddAssign, SubAssign,
)]
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
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }
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

    pub fn rectangle(&self, other: &Self) -> [Coord; 4] {
        let (min_x, max_x) = (min(self.x, other.x), max(self.x, other.x));
        let (min_y, max_y) = (min(self.y, other.y), max(self.y, other.y));

        [
            Coord { x: min_x, y: min_y }, // BL
            Coord { x: max_x, y: min_y }, // BR
            Coord { x: min_x, y: max_y }, // TL
            Coord { x: max_x, y: max_y }, // TR
        ]
    }
}
