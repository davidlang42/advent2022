use std::env;
use std::fs;

fn decode(input: &str) -> isize {
    input.chars().rev().enumerate().map(|(i, c)| 5_isize.pow(i as u32) * match c {
        '=' => -2,
        '-' => -1,
        n => n as isize - '0' as isize
    }).sum()
}

fn encode(input: isize) -> String {
    let mut result = String::new();
    let mut value = input;
    while value > 0 {
        let remainder = (value + 2).rem_euclid(5) - 2;
        result.push(match remainder {
            -2 => '=',
            -1 => '-',
            n => (n + '0' as isize) as u8 as char,
        });
        value = (value - remainder) / 5;
    }
    result.chars().rev().collect()
}

fn _test() {
    for n in 1..21 {
        let encoded = encode(n);
        println!("{}: {} -> {}", n, encoded, decode(&encoded));
    }
}

const NL: &str = "\n";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let sum = text.split(NL).map(decode).sum();
        println!("sum: {}, encoded: {}", sum, encode(sum));
    } else {
        println!("Please provide 1 argument: Filename");
    }
}