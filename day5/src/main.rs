use std::env;
use std::fs;
use std::str::FromStr;

struct Instruction {
    count: usize,
    from: usize,
    to: usize
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let sections: Vec<&str> = text.split("\r\n\r\n").collect();
        let mut stacks = parse_starting_map(sections[0]);
        let instructions: Vec<Instruction> = sections[1].split("\r\n").map(|s| s.parse().unwrap()).collect();
        for i in instructions {
            process_instruction(&mut stacks, i);
        }
        println!("Top of each stack: {}", stacks.iter().map(|s| s[s.len()-1]).collect::<String>());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn parse_starting_map(text: &str) -> Vec<Vec<char>> {
    let mut lines: Vec<&str> = text.split("\r\n").collect();
    lines.reverse();
    let count: usize = lines[0].split("   ").count();
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..count {
        stacks.push(Vec::new());
    }
    for l in 1..lines.len() {
        for s in 0..count {
            let c: char = lines[l].chars().nth(char_index_for_stack(s)).unwrap();
            if c != ' ' {
                stacks[s].push(c);
            }
        }
    }
    stacks
}

fn char_index_for_stack(stack_index: usize) -> usize {
    1 + 4 * stack_index
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        // move 1 from 2 to 1
        let words: Vec<&str> = line.split(" ").collect();
        if words.len() != 6 {
            return Err(format!("Should contain exactly 6 words: {}", line));
        }
        Ok(Instruction {
            count: words[1].parse().unwrap(),
            from: words[3].parse().unwrap(),
            to: words[5].parse().unwrap()
        })
    }
}

fn process_instruction(stacks: &mut Vec<Vec<char>>, i: Instruction) {
    for _ in 0..i.count {
        let c = stacks[i.from-1].pop().unwrap();
        stacks[i.to-1].push(c);
    }
}