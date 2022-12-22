use std::env;
use std::fs;
use std::str::FromStr;
use std::collections::HashMap;

struct Assignment {
    name: String,
    expression: Expression
}
enum Expression {
    Literal(isize),
    Algebra(String, Operation, String),
}

enum Operation {
    Product,
    Sum,
    Difference,
    Division
}

const NL: &str = "\r\n";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut assignments: HashMap<String, Assignment> = HashMap::new();
        for assignment in text.split(NL).map(|s| s.parse::<Assignment>().unwrap()) {
            assignments.insert(assignment.name.to_string(), assignment);
        }
        println!("Root value: {}", assignments["root"].calculate(&assignments));
        // part2
        assignments.insert("root".to_string(), match &assignments["root"].expression {
            Expression::Literal(_) => panic!("Root should have been an operation"),
            Expression::Algebra(a, _, b) => Assignment {
                name: "root".to_string(),
                expression: Expression::Algebra(a.to_string(), Operation::Difference, b.to_string())
            }
        });
        println!("My value: {}", assignments["root"].goal_find(0, &assignments, "humn"));
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

impl FromStr for Expression {
    type Err = String;

    fn from_str(segment: &str) -> Result<Self, Self::Err> {
        //Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
        let words: Vec<&str> = segment.split(" ").collect();
        if words.len() == 1 {
            Ok(Expression::Literal(words[0].parse().unwrap()))
        } else if words.len() == 3 {
            let a = words[0].to_string();
            let b = words[2].to_string();
            let operation = words[1].parse().unwrap();
            Ok(Expression::Algebra(a, operation, b))
        } else {
            Err(format!("Should have 1 or 3 words: {}", segment))
        }
    }
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(symbol: &str) -> Result<Self, Self::Err> {
        match symbol {
            "*" => Ok(Operation::Product),
            "+" => Ok(Operation::Sum),
            "-" => Ok(Operation::Difference),
            "/" => Ok(Operation::Division),
            _ => Err(format!("Operation not found: {}", symbol))
        }
    }
}

impl FromStr for Assignment {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let segments: Vec<&str> = line.split(": ").collect();
        if segments.len() != 2 {
            Err(format!("Expected 2 segments: {}", line))
        } else {
            Ok(Assignment {
                name: segments[0].to_string(),
                expression: segments[1].parse().unwrap()
            })
        }
    }
}

impl Operation {
    fn run(&self, a: isize, b: isize) -> isize{
        match self {
            Operation::Product => a * b,
            Operation::Sum => a + b,
            Operation::Difference => a - b,
            Operation::Division => a / b
        }
    }

    fn inverse(&self) -> Operation {
        match self {
            Operation::Product => Operation::Division,
            Operation::Sum => Operation::Difference,
            Operation::Difference => Operation::Sum,
            Operation::Division => Operation::Product
        }
    }
}

impl Assignment {
    fn calculate(&self, assignments: &HashMap<String, Assignment>) -> isize {
        match &self.expression {
            Expression::Literal(l) => *l,
            Expression::Algebra(a, op, b) => {
                let a_value = assignments[a].calculate(assignments);
                let b_value = assignments[b].calculate(assignments);
                op.run(a_value, b_value)
            }
        }
    }

    fn calculate_without(&self, assignments: &HashMap<String, Assignment>, excluding: &str) -> Option<isize> {
        if self.name == *excluding {
            None
        } else {
            Some(match &self.expression {
                Expression::Literal(l) => *l,
                Expression::Algebra(a, op, b) => {
                    let a_value = assignments[a].calculate_without(assignments, excluding)?;
                    let b_value = assignments[b].calculate_without(assignments, excluding)?;
                    op.run(a_value, b_value)
                }
            })
        }
    }

    fn goal_find(&self, goal: isize, assignments: &HashMap<String, Assignment>, find_value_of: &str) -> isize {
        if self.name == *find_value_of {
            goal
        } else {
            match &self.expression {
                Expression::Literal(_) => panic!("Can't find a goal when '{}' is a literal", self.name),
                Expression::Algebra(a, op, b) => {
                    let a_value = assignments[a].calculate_without(assignments, find_value_of);
                    let b_value = assignments[b].calculate_without(assignments, find_value_of);
                    match (a_value, b_value) {
                        (Some(_), Some(_)) => panic!("Can't find a goal when both sides are literal: {}, {}", a, b),
                        (Some(a_literal), None) => assignments[b].goal_find(op.inverse().run(goal, a_literal), assignments, find_value_of), // when op=diff, might need: (Some(a_literal), None) => expressions[b].goal_find(b, a_literal - goal, expressions, find_value_of),
                        (None, Some(b_literal)) => assignments[a].goal_find(op.inverse().run(goal, b_literal), assignments, find_value_of),
                        (None, None) => panic!("Can't find a goal when both sides are variable: {}, {}", a, b)
                    }
                }
            }
        }
    }
}