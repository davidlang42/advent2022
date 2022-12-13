use std::env;
use std::fs;
use std::str::FromStr;
use core::cmp::Ordering;

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
        let mut pairs: Vec<Vec<Item>> = text.split("\r\n\r\n").map(|pair| pair.split("\r\n").map(|line| line.parse().unwrap()).collect()).collect();
        let mut sum = 0;
        let mut packets: Vec<Item> = vec!["[[2]]".parse().unwrap(), "[[6]]".parse().unwrap()];
        for (i, pair) in pairs.iter_mut().enumerate() {
            if pair.len() != 2 {
                panic!("Pair should contain exactly 2");
            }
            if pair[0].cmp(&pair[1]) == Ordering::Less {
                sum += i + 1;
            }
            packets.append(pair);
        }
        println!("Sum of pair indicies in correct order: {}", sum);
        packets.sort();
        let i1 = packets.iter().position(|p| *p == "[[2]]".parse().unwrap()).unwrap() + 1;
        let i2 = packets.iter().position(|p| *p == "[[6]]".parse().unwrap()).unwrap() + 1;
        println!("Indicies: {} * {} = {}", i1, i2, i1*i2);
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

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self,other) {
            (Item::Int(a_int), Item::Int(b_int)) => a_int.cmp(b_int),
            (Item::List(a_list), Item::List(b_list)) => {
                let mut i = 0;
                while i < a_list.len() && i < b_list.len() {
                    let result = a_list[i].cmp(&b_list[i]);
                    if result != Ordering::Equal {
                        return result;
                    }
                    i += 1;
                }
                a_list.len().cmp(&b_list.len())
            },
            (Item::Int(a_int), Item::List(_)) => Item::List(vec![Item::Int(*a_int)]).cmp(other),
            (Item::List(_), Item::Int(b_int)) => self.cmp(&Item::List(vec![Item::Int(*b_int)]))
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Item {}