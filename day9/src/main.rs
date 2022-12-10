use std::env;
use std::fs;
use std::str::FromStr;
use std::collections::HashSet;

const NL: &str = "\r\n";

enum Direction {
    Right,
    Up,
    Left,
    Down
}

struct Instruction {
    direction: Direction,
    count: usize
}

struct Rope {
    knots: Vec<Point>,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: isize,
    y: isize
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let instructions: Vec<Instruction> = text.split(NL).map(|s| s.parse().unwrap()).collect();
        println!("Unique tail positions for rope length 2: {}", run_simulation(&instructions, 2).len());
        println!("Unique tail positions for rope length 10: {}", run_simulation(&instructions, 10).len());

    } else {
        println!("Please provide 1 argument: Filename");
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = line.split(" ").collect();
        Ok(Instruction {
            direction: match parts[0] {
                "R" => Direction::Right,
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                _ => panic!("Invalid instruction")
            },
            count: parts[1].parse().unwrap()
        })
    }
}

fn move_rope(rope: &mut Rope, direction: &Direction) {
    let (x, y) = match direction {
        Direction::Right => (1, 0),
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0)
    };
    rope.knots[0].x += x;
    rope.knots[0].y += y;
    for i in 1..rope.knots.len() {
        let diff = Point { x: rope.knots[i-1].x - rope.knots[i].x, y: rope.knots[i-1].y - rope.knots[i].y };
        if diff.x.abs() > 1 || diff.y.abs() > 1 {
            rope.knots[i].x += diff.x.signum();
            rope.knots[i].y += diff.y.signum();
        }
    }
}

fn run_simulation(instructions: &Vec<Instruction>, rope_length: usize) -> HashSet<Point> {
    let mut rope = Rope { knots: Vec::new() };
    for _ in 0..rope_length {
        rope.knots.push(Point { x: 0, y: 0});
    }
    let mut visited: HashSet<Point> = HashSet::new();
    for instruction in instructions {
        for _ in 0..instruction.count {
            move_rope(&mut rope, &instruction.direction);
            visited.insert(*rope.knots.last().unwrap());
        }
    }
    visited
}