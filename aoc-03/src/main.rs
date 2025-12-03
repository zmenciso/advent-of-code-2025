use aoc_03::Bank;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output: usize = 0;

    for line in io::stdin().lines() {
        let line = line.expect("Could not read line");
        let bank = Bank::new(&line);
        output += bank.max_joltage();
    }

    println!("{output}");

    Ok(())
}
