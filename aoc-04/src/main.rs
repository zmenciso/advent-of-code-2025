use aoc_04::*;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut floor = Warehouse::new();

    for line in io::stdin().lines() {
        let line = line.expect("Could not read line");
        floor.add_row(&line);
    }

    println!("Initial state:");
    println!("{floor}");

    let mut removed = floor.annotate();
    let mut count = 0usize;

    while removed > 0 {
        println!("Remove {removed} roll(s) of paper:");
        // println!("{floor}");
        floor.clean();

        count += removed;
        removed = floor.annotate();
    }

    println!("{count}");

    Ok(())
}
