use std::error::Error;
use std::io;

fn valid_id(id: &str) -> Result<usize, std::num::ParseIntError> {
    let n = id.len();
    if n < 2 {
        return Ok(0);
    }

    for len in 1..(n / 2) + 1 {
        // Early exit if the substring couldn't repeat evenly
        if n % len != 0 {
            continue;
        }

        let substring = &id[..len];

        if (0..n / len).all(|i| &id[i * len..(i + 1) * len] == substring) {
            return id.parse::<usize>();
        }
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
