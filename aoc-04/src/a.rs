use aoc_04::*;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut floor = Warehouse::new();

    for line in io::stdin().lines() {
        let line = line.expect("Could not read line");
        floor.add_row(&line);
    }

    println!("{floor}");

    let count = floor.annotate();

    println!("{floor}");
    println!("{count}");

    Ok(())
}
