use std::io;

const MOD: i32 = 100;
const START: i32 = 50;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pos: i32 = START;
    let mut count: i32 = 0;

    for line in io::stdin().lines() {
        let line = line.expect("could not read line");

        let digit = line
            .find(|c: char| c.is_ascii_digit())
            .ok_or("no digit in input line")?;

        let (dir, amnt) = line.split_at(digit);
        let amnt = amnt.parse::<i32>()?;
        let amnt = match dir {
            "R" => amnt,
            "L" => amnt * -1,
            _ => {
                println!("Undefined behavior for input {line}");
                amnt
            }
        };

        if amnt >= 0 {
            count += (pos + amnt) / MOD;
        } else {
            let rev = (MOD - pos) % MOD;
            count += (rev - amnt) / MOD;
        }

        pos = (pos + amnt).rem_euclid(MOD);
    }

    println!("{count}");
    Ok(())
}
