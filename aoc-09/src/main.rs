use aoc_09::*;
use std::error::Error;
use std::io;

pub fn on_boundary(point: Coord, line: Segment) -> bool {
    // Vertical edge: p's x matches, y within l
    if line.0.x == line.1.x {
        point.x == line.0.x
            && point.y <= line.0.y.max(line.1.y)
            && point.y >= line.0.y.min(line.1.y)
    }
    // Horizontal edge: p's y matches, x within l
    else if line.0.y == line.0.y {
        point.y == line.0.y
            && point.x <= line.0.x.max(line.1.x)
            && point.x >= line.0.x.min(line.1.x)
    }
    // We shouldn't be here
    else {
        false
    }
}

fn inside_polygon(tiles: &Vec<Coord>, point: Coord) -> bool {
    let n = tiles.len();
    let mut intersections = 0;

    for i in 0..n {
        let (a, b) = (tiles[i], tiles[(i + 1) % n]);

        if on_boundary(point, (a, b)) {
            return true;
        }

        // If there is a point above and below, there might be an intersection
        // (Only if the segment is vertical)
        if (a.y > point.y) != (b.y > point.y) && a.x == b.x {
            // Intersection is right of point
            if a.x > point.x {
                intersections += 1;
            }
        }
    }
    intersections % 2 != 0
}

fn valid_rectangle(tiles: &Vec<Coord>, rectangle: &[Coord; 4]) -> bool {
    rectangle.iter().all(|&p| inside_polygon(tiles, p))
}

fn largest_rec(tiles: &Vec<Coord>) -> (Coord, Coord, Int) {
    let mut max: (Coord, Coord, Int) = (Coord::new(), Coord::new(), 0);

    for (i, &a) in tiles.iter().enumerate() {
        for (j, &b) in tiles.iter().enumerate() {
            if i <= j {
                continue;
            }

            let area = a.area(&b);

            if area > max.2 && valid_rectangle(tiles, &a.rectangle(&b)) {
                max = (a, b, area);
            }
        }
    }

    max
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut tiles: Vec<Coord> = Vec::new();

    for line in io::stdin().lines() {
        let line = line.expect("Could not read line");
        let curr = Coord::from_str(&line)?;
        tiles.push(curr);
    }

    let (a, b, area) = largest_rec(&tiles);

    println!("{a} x {b} --> {area}");

    Ok(())
}
