use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Point {
    x: isize,
    y: isize
}

struct Sensor {
    position: Point,
    beacon: Point,
    search_distance: isize,
    search_min: Point,
    search_max: Point
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let sensors: Vec<Sensor> = text.split("\r\n").map(|s| s.parse().unwrap()).collect();
        let y_search = 2000000;
        let mut x_min = sensors.iter().map(|s| s.position.x - distance(&s.position, &s.beacon)).min().unwrap();
        let mut x_max = sensors.iter().map(|s| s.position.x + distance(&s.position, &s.beacon)).max().unwrap();
        let mut not_beacons = 0;
        for x in x_min..(x_max+1) {
            let p = Point { x, y: y_search };
            if !might_be_beacon(&p, &sensors) {
                not_beacons += 1;
            }
        }
        println!("At y={}, {} cannot be beacons", y_search, not_beacons);

        let y_min = 0;
        let y_max = 4000000;
        x_min = 0;
        x_max = y_max;
        for x in x_min..(x_max+1) {
            if x % 100 == 0 {
                println!("Searching row {}", x);
            }
            for y in y_min..(y_max+1) {
                let p = Point { x, y };
                if might_be_beacon(&p, &sensors) {
                    println!("Possible beacon: ({},{}), with tuning freq: {}", p.x, p.y, p.x*4000000+p.y);
                }
            }
        }
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
        let position: Point = segments[0][10..].parse().unwrap(); //Sensor at x=2, y=18
        let beacon: Point = segments[1][21..].parse().unwrap(); //closest beacon is at x=-2, y=15
        let search_distance = distance(&position, &beacon);
        Ok(Sensor {
            position,
            beacon,
            search_distance,
            search_min: Point {
                x: position.x - search_distance,
                y: position.y - search_distance
            },
            search_max: Point {
                x: position.x + search_distance,
                y: position.y + search_distance
            }
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

fn distance(a: &Point, b: &Point) -> isize {
    (a.x.abs_diff(b.x) + a.y.abs_diff(b.y)).try_into().unwrap()
}

fn might_be_beacon(p: &Point, sensors: &Vec<Sensor>) -> bool {
    for s in sensors {
        if p.x <= s.search_max.x && p.x >= s.search_min.x && p.y <= s.search_max.y && p.y >= s.search_min.y {
            if *p == s.position {
                return false; // definitely a sensor, which is not a beacon
            }
            if *p == s.beacon {
                return true; // definitely a beacon
            }
            if distance(&p, &s.position) <= s.search_distance {
                return false; // definitely not a beacon, but might still be another sensor
            }
        }
    }
    return true; // position not scanned, might be beacon
}