use aoc_09::*;
use std::error::Error;
use std::io;

fn largest_rec(points: &Vec<Coord>) -> (Pair, isize) {
    let mut max: (Pair, isize) = (Pair::new(), 0isize);

    for (i, &a) in points.iter().enumerate() {
        for (j, &b) in points.iter().enumerate() {
            if i <= j {
                continue;
            }

            let pair = Pair { a, b };
            let area = pair.area();

            if area > max.1 {
                max = (pair, area);
            }
        }
    }

    max
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut red_tiles: Vec<Coord> = Vec::new();

    for line in io::stdin().lines() {
        let line = line.expect("Could not read line");
        let p = Coord::from_str(&line)?;
        red_tiles.push(p);
    }

    // println!("{:?}", points);
    let (pair, area) = largest_rec(&red_tiles);
    println!("{pair} | {area}");

    Ok(())
}
