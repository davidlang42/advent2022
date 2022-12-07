use std::env;
use std::fs;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        for line in text.split("\r\n") {
            println!("Result(4): {}", find_unique_string(line, 4));
            println!("Result(14): {}", find_unique_string(line, 14));
        }
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn find_unique_string(line: &str, size: usize) -> isize {
    let mut list: VecDeque<char> = VecDeque::new();
    for (i, c) in line.chars().enumerate() {
        list.push_back(c);
        if list.len() > size {
            list.pop_front();
        }
        if list.iter().collect::<HashSet<&char>>().len() == size {
            return (i + 1).try_into().unwrap();
        }
    }
    -1
}