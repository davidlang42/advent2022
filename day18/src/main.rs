use std::env;
use std::fs;
use std::str::FromStr;
use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
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
        let mut points: HashSet<Point> = text.split("\r\n").map(|s| s.parse().unwrap()).collect();
        println!("Part1: uncovered sides: {}", 6*points.len()-covered_sides(&points));
        for air in find_pockets(&points) {
            points.insert(air);
        }
        println!("Part2: external sides: {}", 6*points.len()-covered_sides(&points));
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
    points.iter().flat_map(|p| adjacent_points(p)).filter(|a| points.contains(a)).count()
}

fn adjacent_points(p: &Point) -> Vec<Point> {
    let mut adjacent = Vec::new();
    for delta in [-1,1] {
        adjacent.push(Point { x: p.x + delta, y: p.y, z: p.z });
        adjacent.push(Point { x: p.x, y: p.y + delta, z: p.z });
        adjacent.push(Point { x: p.x, y: p.y, z: p.z + delta });
    }
    adjacent
}

fn find_pockets(points: &HashSet<Point>) -> HashSet<Point> {
    let mut pockets = HashSet::new();
    let min = Point {
        x: points.iter().map(|p| p.x).min().unwrap(),
        y: points.iter().map(|p| p.y).min().unwrap(),
        z: points.iter().map(|p| p.z).min().unwrap()
    };
    let max = Point {
        x: points.iter().map(|p| p.x).max().unwrap(),
        y: points.iter().map(|p| p.y).max().unwrap(),
        z: points.iter().map(|p| p.z).max().unwrap()
    };
    for x in min.x..(max.x+1) {
        for y in min.y..(max.y+1) {
            for z in min.z..(max.z+1) {
                let p = Point { x, y, z };
                if !points.contains(&p) && !pockets.contains(&p) {
                    let mut visited = HashSet::new();
                    if is_sealed(&p, &points, &mut visited, &min, &max) {
                        for v in visited {
                            pockets.insert(v);
                        }
                    }
                }
            }
        }
    }
    pockets
}

fn is_sealed(p: &Point, points: &HashSet<Point>, visited: &mut HashSet<Point>, min: &Point, max: &Point) -> bool {
    if p.x > max.x || p.y > max.y || p.z > max.z || p.x < min.x || p.y < min.y || p.z < min.z {
        // reached the edge
        return false;
    }
    if !visited.insert(*p) {
        // already been here
        return true;
    }
    for a in adjacent_points(p) {
        if !points.contains(&a) && !is_sealed(&a, points, visited, min, max) {
            // found a way to the edge
            return false;
        }
    }
    // found something on every side
    return true;
}