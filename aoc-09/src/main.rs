use aoc_09::*;
use std::error::Error;
use std::io;

fn print_floor(tiles: &Vec<Coord>) {

}

fn main() -> Result<(), Box<dyn Error>> {
    let mut tiles: Vec<Coord> = Vec::new();

    for line in io::stdin().lines() {
        let line = line.expect("Could not read line");
        let p = Coord::from_str(&line)?;
        tiles.push(p);
    }

    Ok(())
}
