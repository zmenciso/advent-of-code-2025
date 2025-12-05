use derive_more::{Add, AddAssign, Sub, SubAssign};
use std::fmt;

#[allow(dead_code)]
const PAPER: char = '@';
const EMPTY: char = '.';
const ACCESSIBLE: char = 'x';
const STRENGTH: usize = 4;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Add, AddAssign, Sub, SubAssign)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn adjacent(&self, n: isize, m: isize) -> Vec<Point> {
        const OFFSETS: [(isize, isize); 8] = [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];

        OFFSETS
            .iter()
            .map(|&(dx, dy)| Point {
                x: self.x + dx,
                y: self.y + dy,
            })
            .filter(|p| p.x >= 0 && p.x < n && p.y >= 0 && p.y < m)
            .collect()
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn points(n: isize, m: isize) -> impl Iterator<Item = Point> {
    (0..m).flat_map(move |y| (0..n).map(move |x| Point { x, y }))
}

#[derive(Debug)]
pub struct Warehouse {
    pub n: isize,
    pub m: isize,
    tiles: Vec<Vec<char>>,
}

impl Warehouse {
    pub fn new() -> Warehouse {
        Warehouse {
            n: 0,
            m: 0,
            tiles: Vec::new(),
        }
    }

    pub fn lookup(&self, loc: &Point) -> char {
        self.tiles[loc.y as usize][loc.x as usize]
    }

    pub fn edit(&mut self, loc: &Point, c: char) {
        self.tiles[loc.y as usize][loc.x as usize] = c;
    }

    pub fn match_adjacent(&self, loc: Point, c: char, exclude: bool) -> Vec<Point> {
        let mut search = loc.adjacent(self.n, self.m);

        match exclude {
            true => search.retain(|p| self.lookup(p) != c),
            false => search.retain(|p| self.lookup(p) == c),
        }

        search
    }

    pub fn add_row(&mut self, line: &str) {
        match self.m {
            0 => self.m = line.len() as isize,
            _ => assert!(self.m == line.len() as isize, "warehouse not rectangular"),
        }

        let row: Vec<char> = line.chars().collect();
        self.tiles.push(row);
        self.n += 1;
    }

    // Replaces @ with x for accessible rolls
    pub fn annotate(&mut self) -> usize {
        let mut count = 0usize;

        for loc in points(self.n, self.m) {
            let targets = self.match_adjacent(loc, EMPTY, true);
            if targets.len() < STRENGTH && self.lookup(&loc) == PAPER {
                self.edit(&loc, ACCESSIBLE);
                count += 1;
            }
            // targets.into_iter().for_each(|p| self.edit(&p, ACCESSIBLE));
        }

        count
    }
}

impl std::fmt::Display for Warehouse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.m {
            for x in 0..self.n {
                let loc = Point { x, y };
                write!(f, "{}", self.lookup(&loc))?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
