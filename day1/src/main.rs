use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        println!("Max sum: {}", find_max_sum(&text));
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn find_max_sum(text: &String) -> u32 {
    let mut sum: u32 = 0;
    let mut max: u32 = 0;
    for s in text.split("\r\n") {
        if s.len() == 0 {
            if sum > max {
                max = sum;
            }
            sum = 0;
        } else {
            let value: u32 = s.parse().expect(&format!("Error parsing number {}", s));
            sum += value;
        }
    }
    if sum > max {
        max = sum;
    }
    return max;
}