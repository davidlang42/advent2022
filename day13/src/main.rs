use std::env;
use std::fs;
use std::str::FromStr;

enum Item {
    Int(isize),
    List(Vec<Item>)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let pairs: Vec<Vec<Item>> = text.split("\r\n\r\n").map(|pair| pair.split("\r\n").map(|line| line.parse().unwrap()).collect()).collect();
        let mut sum = 0;
        for (i, pair) in pairs.iter().enumerate() {
            if pair.len() != 2 {
                panic!("Pair should contain exactly 2");
            }
            if compare(&pair[0], &pair[1]) < 0 {
                sum += i + 1;
            }
        }
        println!("Sum of pair indicies in correct order: {}", sum);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

impl FromStr for Item {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        // [[1],[2,3,4]]
        if line.len() == 0 {
            Ok(Item::List(Vec::new()))
        } else if line.chars().next().unwrap() != '[' || line.chars().last().unwrap() != ']' {
            Ok(Item::Int(line.parse().unwrap()))
        } else {
            Ok(Item::List(split_by_comma(&line[1..line.len()-1]).iter().map(|s| s.parse().unwrap()).collect()))
        }
    }
}

fn split_by_comma(line: &str) -> Vec<String> {
    let mut depth = 0;
    let mut strings: Vec<String> = Vec::new();
    let mut current: Vec<char> = Vec::new();
    for c in line.chars() {
        match c {
            '[' => {
                depth += 1;
                current.push(c);
            }
            ']' => {
                depth -= 1;
                current.push(c);
            },
            ',' if depth == 0 => {
                strings.push(current.iter().collect());
                current = Vec::new();
            },
            _ => current.push(c),
        }
    }
    strings.push(current.iter().collect());
    strings
}

// a < b => -
// a == b => 0
// a > b => +
fn compare(a: &Item, b: &Item) -> isize {
    match (a,b) {
        (Item::Int(a_int), Item::Int(b_int)) => a_int - b_int,
        (Item::List(a_list), Item::List(b_list)) => {
            let mut i = 0;
            while i < a_list.len() && i < b_list.len() {
                let result = compare(&a_list[i], &b_list[i]);
                if result != 0 {
                    return result;
                }
                i += 1;
            }
            (a_list.len() as isize) - (b_list.len() as isize)
        },
        (Item::Int(a_int), Item::List(_)) => compare(&Item::List(vec![Item::Int(*a_int)]), b),
        (Item::List(_), Item::Int(b_int)) => compare(a, &Item::List(vec![Item::Int(*b_int)]))
    }
}