use aoc_08::*;
use ordered_float::OrderedFloat;
use std::collections::HashMap;
use std::error::Error;
use std::io;

const CONNECTIONS: usize = 1000;
const MULT: usize = 3;

fn combinations(coords: &Vec<Coord>) -> HashMap<Pair, Float> {
    let mut distances = HashMap::new();
    let n = coords.len();

    for i in 0..n {
        for j in 0..n {
            if j <= i {
                continue;
            }

            let (a, b) = (coords[i], coords[j]);
            distances.insert((a, b), a.distance(b));
        }
    }

    distances
}

fn circuits(distances: &HashMap<Pair, Float>) -> Vec<usize> {
    let mut ds = DisjointSet::new();

    let mut v: Vec<(&Pair, &Float)> = distances.iter().collect();
    v.sort_by_key(|(_, dist)| OrderedFloat(**dist));

    for ((junc_a, junc_b), _) in v.iter().take(CONNECTIONS) {
        ds.add_pair(*junc_a, *junc_b);
    }

    ds.lengths()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut coords: Vec<Coord> = Vec::new();

    for line in io::stdin().lines() {
        let line = line.expect("Could not read line");
        coords.push(Coord::from_str(line.trim())?);
    }

    let distances = combinations(&coords);
    let mut circuits = circuits(&distances);
    circuits.sort();
    circuits.reverse();

    let product: usize = circuits.iter().take(MULT).product();

    println!("{:?}", circuits);
    println!("{product}");

    Ok(())
}
