use aoc_05::*;
use std::collections::HashSet;
use std::error::Error;
use std::io;

fn add_ids(ids: &mut HashSet<usize>, line: &str) -> Result<(), Box<dyn Error>> {
    let (start, end) = line.split_once(DELIM).ok_or_else(|| "malformed range")?;
    let (start, end) = (start.parse::<usize>()?, end.parse::<usize>()?);

    for num in start..end + 1 {
        ids.insert(num);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut ids: HashSet<usize> = HashSet::new();

    for line in io::stdin().lines() {
        let line = line.expect("Could not read line");
        let line = line.trim();

        if line.contains(DELIM) {
            add_ids(&mut ids, line)?;
        }
    }

    println!("{}", ids.len());

    Ok(())
}
