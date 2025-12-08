use aoc_07::*;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut manifold = Manifold::new();

    for line in io::stdin().lines() {
        let line = line.expect("Could not read line");
        manifold.read_line(&line);
    }

    println!("{manifold}");
    let splits = manifold.simulate();

    println!("{manifold}");
    println!("{splits}");

    Ok(())
}
