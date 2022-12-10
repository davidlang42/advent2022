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
    head: Point,
    tail: Point
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
        let mut rope = Rope { head: Point { x: 0, y: 0}, tail: Point { x: 0, y: 0 }};
        let mut visited: HashSet<Point> = HashSet::new();
        for instruction in instructions {
            for _ in 0..instruction.count {
                move_rope(&mut rope, &instruction.direction);
                visited.insert(rope.tail);
            }
        }
        println!("Unique tail positions: {}", visited.len());
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
    let (x,y) = match direction {
        Direction::Right => (1, 0),
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0)
    };
    rope.head.x += x;
    rope.head.y += y;
    let diff = Point { x: rope.head.x - rope.tail.x, y: rope.head.y - rope.tail.y };
    if diff.x.abs() > 1 || diff.y.abs() > 1 {
        rope.tail.x += diff.x.signum();
        rope.tail.y += diff.y.signum();
    }
}