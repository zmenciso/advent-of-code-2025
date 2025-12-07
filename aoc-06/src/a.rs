use aoc_06::*;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut problem = Problem::new();

    for line in io::stdin().lines() {
        let line = line.expect("Could not read line");
        problem.add_line(&line)?;
    }

    // println!("{:?}", problem);
    println!("{:?}", problem.compute().iter().sum::<usize>());

    Ok(())
}
