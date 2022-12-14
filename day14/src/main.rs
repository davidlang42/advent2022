use std::env;
use std::fs;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Point {
    right: usize,
    down: usize
}

enum Fill {
    Rock,
    Sand
}

struct Path {
    points: Vec<Point>
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let paths: Vec<Path> = text.split("\r\n").map(|s| s.parse().unwrap()).collect();
        let mut grid: HashMap<Point,Fill> = HashMap::new();
        for path in paths {
            path.draw_rock(&mut grid);
        }
        println!("Initial rocks: {}", grid.len());
        let lowest_rock = grid.iter().map(|(p, f)| p.down).max().unwrap();
        println!("Lowest rock: {}", lowest_rock);
        let start = Point { right: 500, down: 0 };
        let mut sand_count = 0;
        while let Some(_) = fall_sand(&mut grid, start, lowest_rock) {
            sand_count += 1;
        }
        println!("Number of sand: {}", sand_count);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

impl FromStr for Path {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        Ok(Path {
            points: line.split(" -> ").map(|p| p.parse().unwrap()).collect()
        })
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(segment: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<&str> = segment.split(",").collect();
        if numbers.len() != 2 {
            panic!("Should have 2 numbers")
        } else {
            Ok(Point {
                right: numbers[0].parse().unwrap(),
                down: numbers[1].parse().unwrap()
            })
        }
    }
}

impl Path {
    fn draw_rock(&self, grid: &mut HashMap<Point, Fill>) {
        for i in 1..self.points.len() {
            let a = &self.points[i - 1];
            let b = &self.points[i];
            for r in if a.right < b.right { a.right..b.right+1 } else { b.right..a.right+1 } {
                for d in if a.down < b.down { a.down..b.down+1 } else { b.down..a.down+1 } {
                    grid.insert(Point { right: r, down: d }, Fill::Rock);
                }
            }
        }
    }
}

fn fall_sand(grid: &mut HashMap<Point, Fill>, p: Point, lowest_rock: usize) -> Option<Point> {
    if p.down >= lowest_rock {
        // sand falls into the abyss
        None
    } else {
        let down_centre = Point { right: p.right, down: p.down + 1 };
        if !grid.contains_key(&down_centre) {
            return fall_sand(grid, down_centre, lowest_rock);
        }
        if (p.right == 0) {
            panic!("Reached right == 0");
        }
        let down_left = Point { right: p.right - 1, down: p.down + 1 };
        if !grid.contains_key(&down_left) {
            return fall_sand(grid, down_left, lowest_rock);
        }
        let down_right = Point { right: p.right + 1, down: p.down + 1 };
        if !grid.contains_key(&down_right) {
            return fall_sand(grid, down_right, lowest_rock);
        }
        grid.insert(p, Fill::Sand);
        Some(p)
    }
}