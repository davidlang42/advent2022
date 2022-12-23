use std::env;
use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: isize,
    y: isize
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Direction {
    North,
    South,
    West,
    East
}

const NL: &str = "\r\n";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut elves = load_elves(&text);
        let mut next_direction = Direction::North;
        let mut change: bool;
        let rounds: usize = args[2].parse().unwrap();
        println!("Before:");
        _display_grid(&elves);
        println!("");
        for round in 1..(rounds+1) {
            (elves, change) = process_round(&elves, &mut next_direction);
            if !change {
                // finished
                println!("No moves required in round {}", round);
                break;
            }
            let (min, _) = find_bounds(&elves);
            println!("After round {}: ({},{})", round, min.x, min.y);
            //_display_grid(&elves);
            //println!("");
        }
        let size = find_size(&elves);
        println!("{} in {}x{} grid with {} empty places", elves.len(), size.x, size.y, size.x*size.y-elves.len() as isize);
    } else {
        println!("Please provide 2 arguments: Filename, Rounds");
    }
}

fn find_bounds(points: &HashSet<Point>) -> (Point, Point) {
    (Point {
        x: points.iter().map(|p| p.x).min().unwrap(),
        y: points.iter().map(|p| p.y).min().unwrap()
    },
    Point {
        x: points.iter().map(|p| p.x).max().unwrap(),
        y: points.iter().map(|p| p.y).max().unwrap()
    })
}

fn find_size(points: &HashSet<Point>) -> Point {
    let (min, max) = find_bounds(points);
    Point {
        x: max.x - min.x + 1,
        y: max.y - min.y + 1
    }
}

fn load_elves(text: &str) -> HashSet<Point> {
    let mut points = HashSet::new();
    for (y, line) in text.split(NL).enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                points.insert(Point { x: x as isize, y: y as isize });
            }
        }
    }
    points
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::East,
            Direction::East => Direction::North
        }
    }

    fn forward(&self, p: &Point) -> Point {
        match self {
            Direction::North => Point { x: p.x, y: p.y - 1 },
            Direction::South => Point { x: p.x, y: p.y + 1 },
            Direction::West => Point { x: p.x - 1, y: p.y },
            Direction::East => Point { x: p.x + 1, y: p.y }
        }
    }

    fn corners(&self, p: &Point) -> (Point, Point) {
        match self {
            Direction::North => (Point { x: p.x - 1, y: p.y - 1 },Point { x: p.x + 1, y: p.y - 1 }),
            Direction::South => (Point { x: p.x - 1, y: p.y + 1 },Point { x: p.x + 1, y: p.y + 1 }),
            Direction::West => (Point { x: p.x - 1, y: p.y - 1 },Point { x: p.x - 1, y: p.y + 1 }),
            Direction::East => (Point { x: p.x + 1, y: p.y - 1 },Point { x: p.x + 1, y: p.y + 1 })
        }
    }
}

fn process_round(old: &HashSet<Point>, direction: &mut Direction) -> (HashSet<Point>, bool) {
    let mut moves: HashMap<Point, Point> = HashMap::new(); // key: new point, value: old point
    let mut conflicts: HashSet<Point> = HashSet::new();
    let mut change = false;
    for old_p in old {
        if clear_all_sides(old, old_p) {
            // no need to move
            moves.insert(*old_p, *old_p);
        } else {
            change = true;
            let new_p = propose_move(old, old_p, direction);
            //println!("({},{}) proposes move to ({},{})", old_p.x, old_p.y, new_p.x, new_p.y);
            if conflicts.contains(&new_p) {
                // existing conflict, don't move this point
                if moves.insert(*old_p, *old_p) != None {
                    panic!("Double conflict not moving ({},{})", old_p.x, old_p.y);
                }
                panic!("This really was required after all");
            } else if let Some(conflict) = moves.insert(new_p, *old_p) {
                // new conflict, don't move this point, and move the conflict back as well
                //println!("Found conflict at ({},{}), therefore ({},{}) and ({},{}) don't move", new_p.x, new_p.y, old_p.x, old_p.y, conflict.x, conflict.y);
                if moves.remove(&new_p) == None {
                    panic!("Failed to remove conflict");
                }
                if moves.insert(*old_p, *old_p) != None {
                    panic!("Double conflict not moving ({},{})", old_p.x, old_p.y);
                }
                if moves.insert(conflict, conflict) != None {
                    panic!("Double conflict not moving ({},{})", conflict.x, conflict.y);
                }
                conflicts.insert(new_p);
            }
        }
    }
    *direction = direction.next();
    (moves.into_iter().map(|(k,_)| k).collect(), change)
}

fn propose_move(existing: &HashSet<Point>, p: &Point, first_direction: &Direction) -> Point {
    let mut direction = *first_direction;
    loop {
        let new_p = direction.forward(p);
        let (c1, c2) = direction.corners(p);
        //println!("({},{}) checks {:?}", p.x, p.y, direction);
        if existing.contains(&new_p) {
            //println!("No, because centre ({},{}) is taken", new_p.x, new_p.y);
        } else if existing.contains(&c1) {
            //println!("No, because left corner ({},{}) is taken", c1.x, c1.y);
        } else if existing.contains(&c2) {
            //println!("No, because right corner ({},{}) is taken", c2.x, c2.y);
        } else {
            return new_p;
        }
        direction = direction.next();
        if direction == *first_direction {
            break;
        }
    }
    *p // if no moves available, don't move
}

fn clear_all_sides(existing: &HashSet<Point>, p: &Point) -> bool {
    for x in (p.x-1)..(p.x+2) {
        for y in (p.y-1)..(p.y+2) {
            let check = Point { x, y };
            if *p != check && existing.contains(&check) {
                return false;
            }
        }
    }
    true
}

fn _display_grid(points: &HashSet<Point>) {
    let (min, max) = find_bounds(points);
    for y in min.y..(max.y+1) {
        let mut line = Vec::new();
        for x in min.x..(max.x+1) {
            line.push(if points.contains(&Point { x, y }) {
                '#'
            } else {
                '.'
            });
        }
        println!("{}", line.iter().collect::<String>());
    }
}