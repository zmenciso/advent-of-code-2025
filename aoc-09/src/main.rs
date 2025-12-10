use aoc_09::*;
use std::error::Error;
use std::io;

fn tile_inside(tiles: &Vec<Coord>, rectangle: [Coord; 4]) -> bool {
    // rectangle[0] is always the bottom left, rectangle[3] is always the top right
    let (min_x, min_y) = (rectangle[0].x, rectangle[0].y);
    let (max_x, max_y) = (rectangle[3].x, rectangle[3].y);

    // Strictly inside, not on --> no equal then
    tiles
        .iter()
        .all(|&p| p.x > min_x && p.y > min_y && p.x < max_x && p.y < max_y)
}

fn valid_rectangle(tiles: &Vec<Coord>, a: &Coord, b: &Coord) -> bool {
    let r = a.rectangle(b);

    // False if:
    //   Rectangle is entirely outside boundary - 3 corners red
    //   Rectangle has red tile inside
    let red_corners = r.iter().filter(|&p| tiles.contains(p)).count();
    let tile_inside = tile_inside(tiles, r);
    println!("{a} x {b} --> {red_corners} red corners, tile inside {tile_inside}");
    if red_corners == 3 || tile_inside {
        return false;
    }

    true
}

fn largest_rec(tiles: &Vec<Coord>) -> (Coord, Coord, Int) {
    let mut max: (Coord, Coord, Int) = (Coord::new(), Coord::new(), 0);

    for (i, &a) in tiles.iter().enumerate() {
        for (j, &b) in tiles.iter().enumerate() {
            if i <= j {
                continue;
            }

            let area = a.area(&b);

            if area > max.2 && valid_rectangle(tiles, &a, &b) {
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
