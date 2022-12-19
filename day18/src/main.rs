use std::env;
use std::fs;
use std::str::FromStr;
use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
    z: isize
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let points: HashSet<Point> = text.split("\r\n").map(|s| s.parse().unwrap()).collect();
        println!("Part1: uncovered sides: {}", 6*points.len()-covered_sides(&points));
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let coordinates: Vec<&str> = text.split(",").collect();
        if coordinates.len() != 3 {
            Err(format!("Should have had 3 ordinates separated by commas"))
        } else {
            Ok(Point {
                x: coordinates[0].parse().unwrap(),
                y: coordinates[1].parse().unwrap(),
                z: coordinates[2].parse().unwrap()
            })
        }
    }
}

fn covered_sides(points: &HashSet<Point>) -> usize {
    let mut count = 0;
    for p in points.iter() {
        for delta in [-1,1] {
            if points.contains(&Point { x: p.x + delta, y: p.y, z: p.z }) {
                count += 1;
            }
            if points.contains(&Point { x: p.x, y: p.y + delta, z: p.z }) {
                count += 1;
            }
            if points.contains(&Point { x: p.x, y: p.y, z: p.z + delta }) {
                count += 1;
            }
        }
    }
    count
}