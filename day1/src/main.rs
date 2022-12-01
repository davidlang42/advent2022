use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut sums = find_sums(&text);
        sums.sort();
        println!("Max sum: {}", sums.last().expect("will have at least 1"));
        println!("Top 3 sum: {}", sums.iter().rev().take(3).sum::<u32>());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn find_sums(text: &String) -> Vec<u32> {
    let mut sum: u32 = 0;
    let mut list: Vec<u32> = Vec::new();
    for s in text.split("\r\n") {
        if s.len() == 0 {
            list.push(sum);
            sum = 0;
        } else {
            let value: u32 = s.parse().expect(&format!("Error parsing number {}", s));
            sum += value;
        }
    }
    list.push(sum);
    return list;
}