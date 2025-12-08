use aoc_07::*;
use std::collections::HashMap;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut manifold = Manifold::new();

    for line in io::stdin().lines() {
        let line = line.expect("Could not read line");
        manifold.read_line(&line);
    }

    println!("{manifold}");

    let mut splits = HashMap::new();
    simulate(&manifold, &mut splits, manifold.start);

    if let Some((_, val)) = splits.iter().min_by_key(|(p, _)| p.y) {
        println!("{}", *val);
    }

    Ok(())
}
