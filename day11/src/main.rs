use std::collections::VecDeque;
use std::env;
use std::fs;
use std::str::FromStr;

struct Monkey {
    throw_count: u32,
    items: VecDeque<f32>,
    operation: Operation,
    test: Test
}

enum Operation {
    Multiply(f32),
    Add(f32),
    Square
}

struct Test {
    divisible_by: f32,
    true_index: usize,
    false_index: usize
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        // part1
        let mut monkies: Vec<Monkey> = text.split("\r\n\r\n").map(|s| s.parse().unwrap()).collect();
        // for _ in 0..20 {
        //     run_round(&mut monkies, 3);
        // }
        // let mut throws: Vec<f32> = monkies.iter().map(|m| m.throw_count).collect();
        // throws.sort();
        // throws.reverse();
        // println!("Monkey business part1: {}", throws.iter().take(2).product::<f32>());
        // part2
        // monkies = text.split("\r\n\r\n").map(|s| s.parse().unwrap()).collect();
        for _ in 0..20 {
            run_round(&mut monkies, 1.0);
        }
        for (i, m) in monkies.iter().enumerate() {
            println!("Monky {} has thrown {} items", i, m.throw_count);
        }
        let mut throws: Vec<u32> = monkies.iter().map(|m| m.throw_count).collect();
        throws.sort();
        throws.reverse();
        println!("Monkey business part2: {}", throws.iter().take(2).product::<u32>());
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
                "+" => Operation::Multiply(2.0),
                _ => panic!("Invalid operation")
            }
        } else {
            let literal: f32 = words[4].parse().unwrap();
            match words[3] {
                "*" => Operation::Multiply(literal),
                "+" => Operation::Add(literal),
                _ => panic!("Invalid operation")
            }
        })
    }
}

fn run_round(monkies: &mut Vec<Monkey>, worry_decrease_factor: f32) {
    for i in 0..monkies.len() {
        let mut self_throw: VecDeque<f32> = VecDeque::new();
        while let Some(item) = monkies[i].items.pop_front() {
            let new_item = run_operation(item, &monkies[i].operation, worry_decrease_factor);
            let new_monkey = run_test(new_item, &monkies[i].test);
            monkies[new_monkey].items.push_back(new_item);
            monkies[i].throw_count += 1;
        }
        monkies[i].items.append(&mut self_throw);
    }
}

fn run_operation(old: f32, operation: &Operation, worry_decrease_factor: f32) -> f32 {
    let new = match operation {
        Operation::Square => old * old,
        Operation::Add(delta) => old + delta,
        Operation::Multiply(factor) => old * factor
    };
    new / worry_decrease_factor
}

fn run_test(value: f32, test: &Test) -> usize {
    if value % test.divisible_by == 0.0 {
        test.true_index
    } else {
        test.false_index
    }
}