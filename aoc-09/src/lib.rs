use std::error::Error;
use std::fmt;

const DELIM: char = ',';

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, PartialOrd, Ord)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Coord {
    pub fn from_str(input: &str) -> Result<Self, Box<dyn Error>> {
        let split = input.splitn(3, DELIM);
        let mut vals = split.map(|s| s.parse::<isize>().unwrap_or(0));

        Ok(Self {
            x: vals.next().ok_or("missing x value")?,
            y: vals.next().ok_or("missing y value")?,
        })
    }

    pub fn distance_squared(&self, other: &Coord) -> isize {
        (self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y)
    }

    pub fn cross_product(&self, a: &Coord, b: &Coord) -> isize {
        (a.x - self.x) * (b.y - self.y) - (a.y - self.y) * (b.x - self.x)
    }
}
