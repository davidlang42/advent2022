use std::env;
use std::fs;
use std::str::FromStr;

struct Range {
    start: u32,
    end: u32
}

struct Pair {
    a: Range,
    b: Range
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let pairs: Vec<Pair> = text.split("\r\n").map(|s| s.parse().unwrap()).collect();
        let count = pairs.iter().filter(|p| full_overlap(&p.a,&p.b) || full_overlap(&p.b,&p.a)).count();
        println!("Count of full overlap within pairs: {}", count);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn full_overlap(outer: &Range, inner: &Range) -> bool {
    inner.start >= outer.start && inner.end <= outer.end
}

impl FromStr for Pair {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let ranges: Vec<&str> = line.split(",").collect();
        if ranges.len() != 2 {
            return Err(format!("Should contain exactly one comma: {}", line));
        }
        Ok(Pair {
            a: ranges[0].parse().unwrap(),
            b: ranges[1].parse().unwrap()
        })
    }
}

impl FromStr for Range {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<&str> = line.split("-").collect();
        if numbers.len() != 2 {
            return Err(format!("Should contain exactly one dash: {}", line));
        }
        Ok(Range {
            start: numbers[0].parse().unwrap(),
            end: numbers[1].parse().unwrap()
        })
    }
}