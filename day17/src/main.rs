use std::env;
use std::fs;
use std::cmp;
use std::collections::HashSet;
use std::collections::HashMap;

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
        let jets: Vec<Direction> = text.chars().map(|c| parse_char(c)).collect();
        let mut chamber: HashSet<Point> = HashSet::new();
        let width = 7;
        let max: usize =  args[2].parse().unwrap();
        let mut r = 0;
        let mut j = 0;
        let mut clean_floors: HashMap<(usize, usize),(usize, isize)> = HashMap::new(); // Mapping (rock index, jet index) to the (rock number, chamber height) of last time a clean floor occurred with these parameters
        let mut floor = 0;
        let mut i = 0;
        while i < max {
            if i % 1000 == 0 {
                println!("{}/{}", i, max);
            }
            let height = add_rock(&mut chamber, width, floor, &rocks, &mut r, &jets, &mut j);
            if full_row(&chamber, width, height) {
                println!("Full row at {}", i);
                floor = height;
                if let Some((existing_i, existing_height)) = clean_floors.insert((r, j), (i, height)) {
                    println!("Pattern found between rows {}({}) and {}({})", existing_i, existing_height, i, height);
                    let delta_i = i - existing_i;
                    let delta_height = height - existing_height;
                    while i + delta_i < max {
                        i += delta_i;
                        floor += delta_height;
                    }
                }
            }
            i += 1;
        }
        println!("Chamber height after {} rocks: {}", max, measure_height(&chamber));
    } else {
        println!("Please provide 2 arguments: Filename, Max height");
    }
}

fn full_row(chamber: &HashSet<Point>, width: isize, up: isize) -> bool {
    for right in 0..width {
        if chamber.get(&Point { right, up }) == None {
            return false;
        }
    }
    true
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

    fn out_of_bounds(&self, width: isize, floor: isize) -> bool {
        self.position.right < 0 || self.position.right + self.size.right > width || self.position.up <= floor
    }
}

impl Point {
    fn new(right: isize, up: isize) -> Point {
        Point { right, up }
    }
}

fn add_rock(chamber: &mut HashSet<Point>, width: isize, floor: isize, rocks: &Vec<Rock>, r: &mut usize, jets: &Vec<Direction>, j: &mut usize) -> isize {
    let mut rock = rocks[*r].clone();
    *r += 1;
    if *r == rocks.len() {
        *r = 0;
    }
    let mut max_up = cmp::max(measure_height(chamber), floor);
    rock.position = Point { right: 2, up: max_up + 4 };
    //println!("{}", draw_chamber(chamber, &rock.absolute(), width).join("\r\n"));
    loop {
        let right_delta = match jets[*j] {
            Direction::Left => -1,
            Direction::Right => 1
        };
        *j += 1;
        if *j == jets.len() {
            *j = 0;
        }
        rock.position.right += right_delta;
        if rock.out_of_bounds(width, floor) || rock.absolute().intersection(chamber).count() > 0 {
            rock.position.right -= right_delta;
        }
        //println!("{}", draw_chamber(chamber, &rock.absolute(), width).join("\r\n"));
        rock.position.up -= 1;
        if rock.out_of_bounds(width, floor) || rock.absolute().intersection(chamber).count() > 0 {
            rock.position.up += 1;
            break;
        }
        //println!("{}", draw_chamber(chamber, &rock.absolute(), width).join("\r\n"));
    }
    for p in rock.absolute() {
        chamber.insert(p);
        if p.up > max_up {
            max_up = p.up;
        }
    }
    //println!("{}", draw_chamber(chamber, &HashSet::new(), width).join("\r\n"));
    max_up
}

fn measure_height(chamber: &HashSet<Point>) -> isize {
    chamber.iter().map(|p| p.up).max().unwrap_or(0)
}

fn _draw_line(chamber: &HashSet<Point>, rock: &HashSet<Point>, up: isize, width: isize) -> String {
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

fn _draw_chamber(chamber: &HashSet<Point>, rock: &HashSet<Point>, width: isize) -> Vec<String> {
    let height = cmp::max(measure_height(chamber), measure_height(rock));
    let mut lines = Vec::new();
    for u in (0..height+1).rev() {
        if u == 0 {
            lines.push(format!("{}: +{}+", u, _draw_line(chamber, rock, u, width)));
        } else {
            lines.push(format!("{}: |{}|", u, _draw_line(chamber, rock, u, width)));
        }
    }
    lines
}