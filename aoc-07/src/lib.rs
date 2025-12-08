use derive_more::{Add, AddAssign, Sub, SubAssign};
use std::collections::{HashMap, HashSet};
use std::fmt;

const START: char = 'S';
const SPLITTER: char = '^';
const EMPTY: char = '.';
#[allow(dead_code)]
const BEAM: char = '|';

const DOWN: Point = Point { x: 0, y: 1 };
const RIGHT: Point = Point { x: 1, y: 0 };

#[derive(Eq, Hash, PartialEq, Add, Sub, AddAssign, SubAssign, Copy, Clone, Debug)]
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
}

pub fn simulate(manifold: &Manifold, splits: &mut HashMap<Point, usize>, loc: Point) -> usize {
    // Base case: Hit the bottom
    if loc.y > manifold.depth {
        return 1;
    }

    // Recursive case: hit a splitter
    if manifold.splitters.contains(&loc) {
        match splits.get(&loc) {
            Some(val) => *val,
            None => {
                let left_branches = simulate(manifold, splits, loc - RIGHT);
                let right_branches = simulate(manifold, splits, loc + RIGHT);
                let branches = left_branches + right_branches;
                splits.insert(loc, branches);
                branches
            }
        }
    }
    // Recursive case: did not hit a splitter
    else {
        simulate(manifold, splits, loc + DOWN)
    }
}

#[derive(PartialEq, Debug, Eq, Hash, Clone)]
pub enum Direction {
    Left,
    Right,
}
