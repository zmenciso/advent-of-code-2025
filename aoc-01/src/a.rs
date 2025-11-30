use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pos: usize = 50;
    let mut count: usize = 0;

    for line in io::stdin().lines() {
        let line = line.expect("could not read line");

        let digit = line
            .find(|c: char| c.is_ascii_digit())
            .ok_or("no digit in input line")?;

        let (dir, amnt) = line.split_at(digit);
        let amnt = amnt.parse::<usize>()?;

        let step = amnt % 100;
        pos = match dir {
            "R" => (pos + step) % 100,
            "L" => ((pos + 100) - step) % 100,
            _ => {
                println!("Undefined behavior for input {line}");
                pos
            }
        };

        if pos == 0 {
            println!("{line}");
            count = count + 1;
        }
    }

    println!("{count}");
    Ok(())
}
