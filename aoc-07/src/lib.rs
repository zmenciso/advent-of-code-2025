use derive_more::{Add, AddAssign, Sub, SubAssign};
use std::collections::HashSet;
use std::error::Error;
use std::fmt;

const START: char = 'S';
const SPLITTER: char = '^';
const EMPTY: char = '.';
const BEAM: char = '|';

const DOWN: Point = Point { x: 0, y: 1 };
const RIGHT: Point = Point { x: 1, y: 0 };

#[derive(Eq, Hash, PartialEq, Add, Sub, AddAssign, SubAssign, Copy, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub struct Manifold {
    pub start: Point,
    pub depth: isize,
    pub length: isize,
    pub splitters: HashSet<Point>,
    pub beams: HashSet<Point>,
}

impl fmt::Display for Manifold {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for p in self.points() {
            if p == self.start {
                write!(f, "{START}")?;
            } else if self.splitters.contains(&p) {
                write!(f, "{SPLITTER}")?;
            } else if self.beams.contains(&p) {
                write!(f, "{BEAM}")?;
            } else {
                write!(f, "{EMPTY}")?;
            }

            if p.x == self.length - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl Manifold {
    pub fn new() -> Manifold {
        Manifold {
            start: Point { x: 0, y: 0 },
            depth: 0,
            length: 0,
            splitters: HashSet::new(),
            beams: HashSet::new(),
        }
    }

    pub fn points(&self) -> impl Iterator<Item = Point> + '_ {
        (0..self.depth).flat_map(move |y| (0..self.length).map(move |x| Point { x: x as isize, y }))
    }

    pub fn read_line(&mut self, line: &str) {
        if self.length == 0 {
            self.length = line.len() as isize;
        }

        assert!(self.length == line.len() as isize, "unexpected line length");

        for (x, c) in line.chars().enumerate() {
            let p = Point {
                x: x as isize,
                y: self.depth,
            };
            match c {
                START => self.start = p,
                SPLITTER => drop(self.splitters.insert(p)),
                _ => {}
            }
        }

        self.depth += 1;
    }

    pub fn simulate(&mut self) -> usize {
        let mut splits = 0usize;

        // Initial beam
        self.beams.insert(self.start + DOWN);

        // Collect all the points so we can mutate
        let points: Vec<Point> = self.points().collect();

        for p in points {
            let above = p - DOWN;

            // At a splitter, only do something if there is a beam above
            if self.splitters.contains(&p) && self.beams.contains(&above) {
                self.beams.insert(p + RIGHT);
                self.beams.insert(p - RIGHT);
                splits += 1;
            }
            // There is a beam above, but that's it
            else if self.beams.contains(&above) {
                self.beams.insert(p);
            }
        }

        splits
    }
}
