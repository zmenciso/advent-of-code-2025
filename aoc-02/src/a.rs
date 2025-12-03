use std::error::Error;
use std::io;

fn valid_id(id: &str) -> Result<usize, std::num::ParseIntError> {
    let n = id.len();
    if n < 2 || n % 2 != 0 {
        return Ok(0);
    }

    if &id[0..n / 2] == &id[n / 2..n] {
        return id.parse::<usize>();
    }

    Ok(0)
}

fn check_ids(start: &str, stop: &str) -> Result<usize, Box<dyn Error>> {
    println!("Start: {start}  Stop: {stop}");
    let mut count: usize = 0;

    let mut next: usize = start.parse()?;
    let stop: usize = stop.parse()?;

    while next <= stop {
        count += valid_id(&next.to_string())?;
        next += 1;
    }

    Ok(count)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut count: usize = 0;
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    for range in line.split(',') {
        if range.is_empty() {
            continue;
        }
        match range.split_once('-') {
            Some((start, stop)) => count += check_ids(start.trim(), stop.trim())?,
            None => println!("Malformed range: {range}"),
        }
    }

    println!("{count}");

    Ok(())
}
