use std::env;
use std::fs;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Point {
    x: isize,
    y: isize
}

struct Sensor {
    position: Point,
    beacon: Point
}

enum Fill {
    Beacon,
    Sensor,
    NoBeacon
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let sensors: Vec<Sensor> = text.split("\r\n").map(|s| s.parse().unwrap()).collect();
        let mut grid: HashMap<Point,Fill> = HashMap::new();
        for (i, sensor) in sensors.iter().enumerate() {
            println!("Sensor {}", i);
            add_to_grid(&mut grid, &sensor);
        }
        let y_search = 2000000;
        let not_beacons = grid.iter().filter(|(p, f)| p.y == y_search && match f {
            Fill::Beacon => false,
            Fill::Sensor => true,
            Fill::NoBeacon => true
        }).count();
        println!("At y={}, {} cannot be beacons", y_search, not_beacons);
        // let mut line: Vec<char> = Vec::new();
        // for x in -4..27 {
        //     line.push(match grid.get(&Point { x, y: y_search }) {
        //         Some(Fill::Beacon) => 'B',
        //         Some(Fill::NoBeacon) => '#',
        //         Some(Fill::Sensor) => 'S',
        //         None => '.'
        //     });
        // }
        // println!("Line {}: {}", y_search, line.iter().collect::<String>());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

impl FromStr for Sensor {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let segments: Vec<&str> = line.split(": ").collect();
        if segments.len() != 2 {
            panic!("Must have 2 segments");
        }
        Ok(Sensor {
            position: segments[0][10..].parse().unwrap(), //Sensor at x=2, y=18
            beacon: segments[1][21..].parse().unwrap(), //closest beacon is at x=-2, y=15
        })
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(segment: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<&str> = segment.split(", ").collect();
        if numbers.len() != 2 {
            panic!("Should have 2 coordinates")
        } else {
            Ok(Point {
                x: numbers[0][2..].parse().unwrap(),//x=2
                y: numbers[1][2..].parse().unwrap()//y=16
            })
        }
    }
}

fn add_to_grid(grid: &mut HashMap<Point,Fill>, sensor: &Sensor) {
    grid.insert(sensor.position, Fill::Sensor); //overwrite
    grid.insert(sensor.beacon, Fill::Beacon); //overwrite
    let max = distance(&sensor.position, &sensor.beacon);
    for x in (sensor.position.x-max)..(sensor.position.x+max) {
        for y in (sensor.position.y-max)..(sensor.position.y+max) {
            let p = Point { x, y };
            if distance(&p, &sensor.position) <= max && !grid.contains_key(&p) {
                grid.insert(p, Fill::NoBeacon); //no overwrite
            }
        }
    }
}

fn distance(a: &Point, b: &Point) -> isize {
    (a.x.abs_diff(b.x) + a.y.abs_diff(b.y)).try_into().unwrap()
}