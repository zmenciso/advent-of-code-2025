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
    else if line.0.y == line.1.y {
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

fn edges_cross(tiles: &Vec<Coord>, rectangle: &[Coord; 5]) -> bool {
    let edges = [
        (rectangle[0], rectangle[1]), // Bottom
        (rectangle[2], rectangle[3]), // Top
        (rectangle[0], rectangle[2]), // Left
        (rectangle[1], rectangle[3]), // Right
    ];

    let n = tiles.len();
    for i in 0..n {
        let edge: Segment = (tiles[i], tiles[(i + 1) % n]);

        for e in edges {
            if segments_cross(edge, e) {
                return false;
            }
        }
    }
    true
}

fn between(v: Int, a: Int, b: Int) -> bool {
    let (min, max) = (a.min(b), a.max(b));
    v > min && v < max
}

fn segments_cross(s1: Segment, s2: Segment) -> bool {
    let s1_vert = s1.0.x == s1.1.x;
    let s2_vert = s2.0.x == s2.1.x;

    // Crossing not possible if both vertical/horizontal
    if s1_vert == s2_vert {
        return false;
    }

    let (vert, horiz) = if s1_vert { (s1, s2) } else { (s2, s1) };

    let vx = vert.0.x;
    let hy = horiz.0.y;

    between(vx, horiz.0.x, horiz.1.x) && between(hy, vert.0.y, vert.1.y)
}

fn valid_rectangle(tiles: &Vec<Coord>, rectangle: &[Coord; 5]) -> bool {
    rectangle.iter().all(|&p| inside_polygon(tiles, p)) && edges_cross(tiles, rectangle)
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
