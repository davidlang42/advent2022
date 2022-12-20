use std::env;
use std::fs;
use std::cmp;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Point {
    right: isize,
    up: isize
}

#[derive(Clone)]
struct Rock {
    size: Point,
    points: HashSet<Point>,
    position: Point
}

enum Direction {
    Left,
    Right
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let rocks: Vec<Rock> = vec![
            Rock::new(vec![
                Point::new(0,0),
                Point::new(1,0),
                Point::new(2,0),
                Point::new(3,0)
            ]),
            Rock::new(vec![
                Point::new(1,0),
                Point::new(0,1),
                Point::new(1,1),
                Point::new(2,1),
                Point::new(1,2),
            ]),
            Rock::new(vec![
                Point::new(0,0),
                Point::new(1,0),
                Point::new(2,0),
                Point::new(2,1),
                Point::new(2,2),
            ]),
            Rock::new(vec![
                Point::new(0,0),
                Point::new(0,1),
                Point::new(0,2),
                Point::new(0,3),
            ]),
            Rock::new(vec![
                Point::new(0,0),
                Point::new(0,1),
                Point::new(1,0),
                Point::new(1,1),
            ])
        ];
        let mut jets: VecDeque<Direction> = text.chars().map(|c| parse_char(c)).collect();
        let mut chamber: HashSet<Point> = HashSet::new();
        let width = 7;
        let max: usize =  args[2].parse().unwrap();
        for i in 0..max {
            if i % 1000 == 0 {
                println!("{}/{}", i, max);
            }
            add_rock(&mut chamber, width, &rocks[i % rocks.len()], &mut jets);
        }
        println!("Chamber height after {} rocks: {}", max, measure_height(&chamber));
    } else {
        println!("Please provide 2 arguments: Filename, Max height");
    }
}

fn parse_char(c: char) -> Direction {
    match c {
        '<' => Direction::Left,
        '>' => Direction::Right,
        _ => panic!("Invalid char")
    }
}

impl Rock {
    fn new(from_points: Vec<Point>) -> Self {
        let mut size = Point::new(0,0);
        let mut points = HashSet::new();
        for p in from_points {
            points.insert(p);
            if p.up > size.up {
                size.up = p.up
            }
            if p.right > size.right {
                size.right = p.right
            }
        }
        size.up += 1;
        size.right += 1;
        Rock {
            position: Point::new(0,0),
            points,
            size
        }
    }

    fn absolute(&self) -> HashSet<Point> {
        let mut abs = HashSet::new();
        for p in &self.points {
            abs.insert(Point {
                right: p.right + self.position.right,
                up: p.up + self.position.up
            });
        }
        abs
    }

    fn out_of_bounds(&self, width: isize) -> bool {
        self.position.right < 0 || self.position.right + self.size.right > width || self.position.up <= 0
    }
}

impl Point {
    fn new(right: isize, up: isize) -> Point {
        Point { right, up }
    }
}

fn add_rock(chamber: &mut HashSet<Point>, width: isize, rock_template: &Rock, jets: &mut VecDeque<Direction>) {
    let mut rock = rock_template.clone();
    rock.position = Point { right: 2, up: measure_height(chamber) + 4 };
    //println!("{}", draw_chamber(chamber, &rock.absolute(), width).join("\r\n"));
    loop {
        let next_jet = jets.pop_front().unwrap();
        let right_delta = match next_jet {
            Direction::Left => -1,
            Direction::Right => 1
        };
        jets.push_back(next_jet);
        rock.position.right += right_delta;
        if rock.out_of_bounds(width) || rock.absolute().intersection(chamber).count() > 0 {
            rock.position.right -= right_delta;
        }
        //println!("{}", draw_chamber(chamber, &rock.absolute(), width).join("\r\n"));
        rock.position.up -= 1;
        if rock.out_of_bounds(width) || rock.absolute().intersection(chamber).count() > 0 {
            rock.position.up += 1;
            break;
        }
        //println!("{}", draw_chamber(chamber, &rock.absolute(), width).join("\r\n"));
    }
    for p in rock.absolute() {
        chamber.insert(p);
    }
    //println!("{}", draw_chamber(chamber, &HashSet::new(), width).join("\r\n"));
}

fn measure_height(chamber: &HashSet<Point>) -> isize {
    chamber.iter().map(|p| p.up).max().unwrap_or(0)
}

fn draw_line(chamber: &HashSet<Point>, rock: &HashSet<Point>, up: isize, width: isize) -> String {
    let mut line = Vec::new();
    for right in 0..width {
        let p = Point { right, up };
        line.push(match (chamber.contains(&p), rock.contains(&p), up) {
            (true, true, _) => panic!("rock & chamber clash"),
            (c, r, 0) if c || r => panic!("found something embedded in floor"),
            (_, _, 0) => '-',
            (true, false, _) => '#',
            (false, true, _) => '@',
            _ => '.'
        });
    }
    line.iter().collect()
}

fn draw_chamber(chamber: &HashSet<Point>, rock: &HashSet<Point>, width: isize) -> Vec<String> {
    let height = cmp::max(measure_height(chamber), measure_height(rock));
    let mut lines = Vec::new();
    for u in (0..height+1).rev() {
        if u == 0 {
            lines.push(format!("{}: +{}+", u, draw_line(chamber, rock, u, width)));
        } else {
            lines.push(format!("{}: |{}|", u, draw_line(chamber, rock, u, width)));
        }
    }
    lines
}