use std::error::Error;
use std::fmt;

const DELIM: char = ',';

#[derive(Debug)]
pub struct Pair {
    pub a: Coord,
    pub b: Coord,
}

impl Pair {
    pub fn new() -> Pair {
        Pair {
            a: Coord { x: 0, y: 0 },
            b: Coord { x: 0, y: 0 },
        }
    }

    pub fn area(&self) -> isize {
        // Add one because the same coord is treated as unit length
        ((self.a.x - self.b.x).abs() + 1) * ((self.a.y - self.b.y).abs() + 1)
    }
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} x {}", self.a, self.b)
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
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
}
