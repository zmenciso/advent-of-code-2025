use aoc_09::*;
use std::collections::HashSet;
use std::error::Error;
use std::io;

fn flood(tiles: &Vec<Coord>) -> HashSet<Coord> {
    let mut interior = HashSet::new();

    interior
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut tiles: Vec<Coord> = Vec::new();

    for line in io::stdin().lines() {
        let line = line.expect("Could not read line");
        let curr = Coord::from_str(&line)?;
        tiles.push(curr);
    }

    println!("{}", tiles.len());

    Ok(())
}
