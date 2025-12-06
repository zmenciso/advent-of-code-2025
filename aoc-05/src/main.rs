use aoc_05::*;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut fresh = Ingredients::new();

    for line in io::stdin().lines() {
        let line = line.expect("Could not read line");
        let line = line.trim();

        // Fresh ingredient ID ranges
        if line.contains(DELIM) {
            fresh.add_range(&line)?;
        }
    }

    println!("{}", fresh.total());

    Ok(())
}
