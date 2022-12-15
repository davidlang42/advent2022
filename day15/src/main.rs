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
        let y_search = 2000000;
        let x_min = sensors.iter().map(|s| s.position.x - distance(&s.position, &s.beacon)).min().unwrap();
        let x_max = sensors.iter().map(|s| s.position.x + distance(&s.position, &s.beacon)).max().unwrap();
        let mut not_beacons = 0;
        for x in x_min..(x_max+1) {
            let p = Point { x, y: y_search };
            if !might_be_beacon(&p, &sensors) {
                not_beacons += 1;
            }
        }
        println!("At y={}, {} cannot be beacons", y_search, not_beacons);
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

fn might_be_beacon(p: &Point, sensors: &Vec<Sensor>) -> bool {
    for s in sensors {
        if *p == s.position {
            return false; // definitely a sensor, which is not a beacon
        }
        if *p == s.beacon {
            return true; // definitely a beacon
        }
        let max = distance(&s.position, &s.beacon);
        if distance(&p, &s.position) <= max {
            return false; // definitely not a beacon, but might still be another sensor
        }
    }
    return true; // position not scanned, might be beacon
}