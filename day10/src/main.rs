use std::env;
use std::fs;
use std::str::FromStr;

const NL: &str = "\r\n";

enum Instruction {
    Noop,
    AddX(isize)
}

#[derive(Copy, Clone)]
struct State {
    x: isize
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let instructions: Vec<Instruction> = text.split(NL).map(|s| s.parse().unwrap()).collect();
        let mut cycles: Vec<State> = vec![State { x: 1 }];
        for instruction in instructions {
            let mut new_states = run_instruction(cycles.last().unwrap(), &instruction);
            cycles.append(&mut new_states);
        }
        let important_cycles = vec![20, 60, 100, 140, 180, 220];
        let mut sum = 0;
        for i in important_cycles {
            println!("Value of X at cycle {}: {}", i, cycles[i-1].x);
            sum += cycles[i-1].x * i as isize;
        }
        println!("Summed signal strengths: {}", sum);
        let mut i = 0;
        while i + 40 < cycles.len() {
            println!("{}", render_line(&cycles[i..i+40]));
            i += 40;
        }
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = line.split(" ").collect();
        Ok(match parts[0] {
            "noop" => Instruction::Noop,
            "addx" => Instruction::AddX(parts[1].parse().unwrap()),
            _ => panic!("Invalid instruction")
        })
    }
}

fn run_instruction(previous: &State, instruction: &Instruction) -> Vec<State> {
    match instruction {
        Instruction::Noop => vec![*previous],
        Instruction::AddX(x) => vec![*previous, State { x: previous.x + x }]
    }
}

fn render_line(cycles: &[State]) -> String {
    cycles.iter().enumerate().map(|(i,s)| if (i as isize - s.x).abs() <= 1 {
        '#'
    } else {
        '.'
    }).collect::<String>()
}