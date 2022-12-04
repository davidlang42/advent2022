use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let priorities: Vec<u32> = text.split("\r\n").map(|s| get_priority(find_common(split_line(s)))).collect();
        println!("Sum of priorities: {}", priorities.iter().sum::<u32>());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn get_priority(c: char) -> u32 {
    if c.is_uppercase() {
        c as u32 - 'A' as u32 + 27
    } else {
        c as u32 - 'a' as u32 + 1
    }
}

fn find_common(sets: Vec<&str>) -> char {
    for c in sets[0].chars() {
        let mut common = true;
        for i in 1..sets.len() {
            if !sets[i].contains(c) {
                common = false;
                break;
            }
        }
        if common {
            return c;
        }
    }
    panic!("No common char found");
}

fn split_line(line: &str) -> Vec<&str> {
    let mut segments: Vec<&str> = Vec::new();
    let half = line.len() / 2;
    segments.push(&line[0..half]);
    segments.push(&line[half..]);
    segments
}