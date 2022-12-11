use std::collections::VecDeque;
use std::env;
use std::fs;
use std::str::FromStr;

struct Monkey {
    throw_count: usize,
    items: VecDeque<usize>,
    operation: Operation,
    test: Test
}

enum Operation {
    Multiply(usize),
    Add(usize),
    Square
}

struct Test {
    divisible_by: usize,
    true_index: usize,
    false_index: usize
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut monkies: Vec<Monkey> = text.split("\r\n\r\n").map(|s| s.parse().unwrap()).collect();
        for _ in 0..20 {
            run_round(&mut monkies);
        }
        let mut throws: Vec<usize> = monkies.iter().map(|m| m.throw_count).collect();
        throws.sort();
        throws.reverse();
        println!("Monkey business: {}", throws.iter().take(2).product::<usize>())
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = text.split("\r\n").collect();
        //Monkey 0:
        //  Starting items: 79, 98
        //  Operation: new = old * 19
        //  Test: divisible by 23
        //    If true: throw to monkey 2
        //    If false: throw to monkey 3
        Ok(Monkey {
            throw_count: 0,
            items: lines[1].split(": ").skip(1).next().unwrap().split(", ").map(|s| s.parse().unwrap()).collect(),
            operation: lines[2].split(": ").skip(1).next().unwrap().parse().unwrap(),
            test: Test {
                divisible_by: lines[3].split(": ").skip(1).next().unwrap().split(" ").skip(2).next().unwrap().parse().unwrap(),
                true_index: lines[4].split(": ").skip(1).next().unwrap().split(" ").skip(3).next().unwrap().parse().unwrap(),
                false_index: lines[5].split(": ").skip(1).next().unwrap().split(" ").skip(3).next().unwrap().parse().unwrap()
            }
        })
    }
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        //new = old * 19
        let words: Vec<&str> = line.split(" ").collect();
        Ok(if words[4].eq("old") {
            match words[3] {
                "*" => Operation::Square,
                "+" => Operation::Multiply(2),
                _ => panic!("Invalid operation")
            }
        } else {
            let literal: usize = words[4].parse().unwrap();
            match words[3] {
                "*" => Operation::Multiply(literal),
                "+" => Operation::Add(literal),
                _ => panic!("Invalid operation")
            }
        })
    }
}

fn run_round(monkies: &mut Vec<Monkey>) {
    for i in 0..monkies.len() {
        while let Some(item) = monkies[i].items.pop_front() {
            let new_item = run_operation(item, &monkies[i].operation);
            let new_monkey = run_test(new_item, &monkies[i].test);
            monkies[new_monkey].items.push_back(new_item);
            monkies[i].throw_count += 1;
        }
    }
}

fn run_operation(old: usize, operation: &Operation) -> usize {
    let new = match operation {
        Operation::Square => old * old,
        Operation::Add(delta) => old + delta,
        Operation::Multiply(factor) => old * factor
    };
    new / 3
}

fn run_test(value: usize, test: &Test) -> usize {
    if value % test.divisible_by == 0 {
        test.true_index
    } else {
        test.false_index
    }
}