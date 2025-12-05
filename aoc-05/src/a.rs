use aoc_05::*;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut fresh = Ingredients::new();
    let mut count = 0usize;

    for line in io::stdin().lines() {
        let line = line.expect("Could not read line");
        let line = line.trim();

        // Fresh ingredient ID ranges
        if line.contains(DELIM) {
            fresh.add_range(&line)?;
        }
        // Available ingredient ID
        else if !line.is_empty() {
            let id = line.parse::<usize>()?;
            if fresh.fresh(&id) {
                println!("{id} fresh");
                count += 1
            } else {
                println!("{id} spoiled");
            }
        }
    }

    println!("{count}");

    Ok(())
}
