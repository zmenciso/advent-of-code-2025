use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::hash::Hash;

pub type Float = f32;
pub type Pair = (Coord, Coord);

const DELIM: char = ',';

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Coord {
    pub fn from_str(input: &str) -> Result<Self, Box<dyn Error>> {
        let split = input.splitn(3, DELIM);
        let mut vals = split.map(|s| s.parse::<isize>().unwrap_or(0));

        Ok(Self {
            x: vals.next().ok_or("missing x value")?,
            y: vals.next().ok_or("missing y value")?,
            z: vals.next().ok_or("missing z value")?,
        })
    }

    pub fn distance(&self, c: Self) -> Float {
        (((c.x - self.x).pow(2) + (c.y - self.y).pow(2) + (c.z - self.z).pow(2)) as f32).sqrt()
    }
}

pub struct DisjointSet<T> {
    pub parents: HashMap<T, T>,
    pub circuits: HashMap<T, usize>,
}

impl<T> DisjointSet<T>
where
    T: Hash + Eq + Clone + Copy,
{
    pub fn new() -> Self {
        Self {
            parents: HashMap::new(),
            circuits: HashMap::new(),
        }
    }

    pub fn add_pair(&mut self, a: T, b: T) {
        let root_a = self.root(&a);
        let root_b = self.root(&b);

        match (root_a, root_b) {
            // New circuit - A is root
            (None, None) => {
                self.parents.insert(a.clone(), a.clone());
                self.parents.insert(b, a.clone());
                self.circuits.insert(a, 2);
            }
            // A already has parent, B does not - B's root becomes A's
            (Some(r_a), None) => {
                self.parents.insert(b, r_a.clone());
                *self.circuits.get_mut(&r_a).unwrap() += 1;
            }
            // B already has parent, A does not - A's root becomes B's
            (None, Some(r_b)) => {
                self.parents.insert(a, r_b.clone());
                *self.circuits.get_mut(&r_b).unwrap() += 1;
            }
            // Both already have parents - merge
            (Some(r_a), Some(r_b)) => {
                if r_a != r_b {
                    if let Some(size_b) = self.circuits.remove(&r_b) {
                        if let Some(size_a) = self.circuits.get_mut(&r_a) {
                            *size_a += size_b;
                        }
                    }
                    self.parents.insert(r_b, r_a);
                }
            }
        }
    }

    pub fn root(&mut self, search: &T) -> Option<T> {
        if !self.parents.contains_key(search) {
            return None;
        }

        let parent = self.parents.get(search).unwrap().clone();

        if parent == *search {
            Some(parent)
        } else {
            let root = self.root(&parent).unwrap();
            // Path compression
            self.parents.insert(search.clone(), root.clone());
            Some(root)
        }
    }

    pub fn contains(&self, search: &T) -> bool {
        self.parents.contains_key(search)
    }

    pub fn lengths(&self) -> Vec<usize> {
        let mut lengths: Vec<usize> = self.circuits.values().copied().collect();
        lengths.sort();
        lengths
    }
}
