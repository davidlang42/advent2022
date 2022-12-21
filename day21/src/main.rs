use std::env;
use std::fs;
use std::str::FromStr;
use std::collections::HashMap;

enum Expression {
    Literal(isize),
    Product(String, String),
    Sum(String, String),
    Difference(String, String),
    Division(String, String)
}

const NL: &str = "\n";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut expressions: HashMap<String, Expression> = text.split(NL).map(|s| parse_expression(s)).collect();
        println!("Expressions: {}", expressions.len());
        println!("Root value: {}", expressions["root"].calculate(&expressions));
        // part2
        expressions.insert("root".to_string(), match &expressions["root"] {
            Expression::Literal(_) => panic!("Root should have been an operation"),
            Expression::Product(a, b) => Expression::Difference(a.to_string(), b.to_string()),
            Expression::Sum(a, b) => Expression::Difference(a.to_string(), b.to_string()),
            Expression::Difference(a, b) => Expression::Difference(a.to_string(), b.to_string()),
            Expression::Division(a, b) => Expression::Difference(a.to_string(), b.to_string())
        });
        println!("My value: {}", expressions["root"].goal_find(0, &expressions, "humn"));
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
            Ok(match words[1] {
                "*" => Expression::Product(a,b),
                "+" => Expression::Sum(a,b),
                "-" => Expression::Difference(a,b),
                "/" => Expression::Division(a,b),
                _ => panic!("operation not found")
            })
        } else {
            panic!("Should have 1 or 3 words");
        }
    }
}

fn parse_expression(line: &str) -> (String, Expression) {
    let segments: Vec<&str> = line.split(": ").collect();
    if segments.len() != 2 {
        panic!("Expected 2 segments");
    }
    (segments[0].to_string(), segments[1].parse().unwrap())
}

impl Expression {
    fn calculate(&self, other_expressions: &HashMap<String, Expression>) -> isize {
        match self {
            Expression::Literal(l) => *l,
            Expression::Product(a, b) => other_expressions[a].calculate(other_expressions) * other_expressions[b].calculate(other_expressions),
            Expression::Sum(a, b) => other_expressions[a].calculate(other_expressions) + other_expressions[b].calculate(other_expressions),
            Expression::Difference(a, b) => other_expressions[a].calculate(other_expressions) - other_expressions[b].calculate(other_expressions),
            Expression::Division(a, b) => other_expressions[a].calculate(other_expressions) / other_expressions[b].calculate(other_expressions)
        }
    }

    fn calculate_without(&self, other_expressions: &HashMap<String, Expression>, excluding: &str) -> Option<isize> {
        match self {
            Expression::Literal(l) => Some(*l),
            Expression::Product(a, b) => {
                if excluding == *a || excluding == *b {
                    None
                } else {
                    Some(other_expressions[a].calculate(other_expressions) * other_expressions[b].calculate(other_expressions))
                }
            },
            Expression::Sum(a, b) => {
                if excluding == *a || excluding == *b {
                    None
                } else {
                    Some(other_expressions[a].calculate(other_expressions) + other_expressions[b].calculate(other_expressions))
                }
            },
            Expression::Difference(a, b) => {
                if excluding == *a || excluding == *b {
                    None
                } else {
                    Some(other_expressions[a].calculate(other_expressions) - other_expressions[b].calculate(other_expressions))
                }
            },
            Expression::Division(a, b) => {
                if excluding == *a || excluding == *b {
                    None
                } else {
                    Some(other_expressions[a].calculate(other_expressions) / other_expressions[b].calculate(other_expressions))
                }
            },
        }
    }

    fn goal_find(&self, goal: isize, expressions: &HashMap<String, Expression>, find_value_of: &str) -> isize {
        match self {
            Expression::Literal(l) => *l,
            Expression::Product(a, b) => {
                let a_value = expressions[a].calculate_without(expressions, find_value_of);
                let b_value = expressions[b].calculate_without(expressions, find_value_of);
                match (a_value, b_value) {
                    (Some(_), Some(_)) => panic!("Can't find a goal when both sides are literal"),
                    (Some(a_literal), None) => expressions[b].goal_find(goal / a_literal, expressions, find_value_of),
                    (None, Some(b_literal)) => expressions[a].goal_find(goal / b_literal, expressions, find_value_of),
                    (None, None) => panic!("Can't find a goal when both sides are variable")
                }
            },
            Expression::Sum(a, b) => {
                let a_value = expressions[a].calculate_without(expressions, find_value_of);
                let b_value = expressions[b].calculate_without(expressions, find_value_of);
                match (a_value, b_value) {
                    (Some(_), Some(_)) => panic!("Can't find a goal when both sides are literal"),
                    (Some(a_literal), None) => expressions[b].goal_find(goal - a_literal, expressions, find_value_of),
                    (None, Some(b_literal)) => expressions[a].goal_find(goal - b_literal, expressions, find_value_of),
                    (None, None) => panic!("Can't find a goal when both sides are variable")
                }
            },
            Expression::Difference(a, b) => {
                let a_value = expressions[a].calculate_without(expressions, find_value_of);
                let b_value = expressions[b].calculate_without(expressions, find_value_of);
                match (a_value, b_value) {
                    (Some(_), Some(_)) => panic!("Can't find a goal when both sides are literal"),
                    (Some(a_literal), None) => expressions[b].goal_find(a_literal - goal, expressions, find_value_of),
                    (None, Some(b_literal)) => expressions[a].goal_find(goal + b_literal, expressions, find_value_of),
                    (None, None) => panic!("Can't find a goal when both sides are variable")
                }
            },
            Expression::Division(a, b) => {
                let a_value = expressions[a].calculate_without(expressions, find_value_of);
                let b_value = expressions[b].calculate_without(expressions, find_value_of);
                match (a_value, b_value) {
                    (Some(_), Some(_)) => panic!("Can't find a goal when both sides are literal"),
                    (Some(a_literal), None) => expressions[b].goal_find(goal * a_literal, expressions, find_value_of),
                    (None, Some(b_literal)) => expressions[a].goal_find(goal * b_literal, expressions, find_value_of),
                    (None, None) => panic!("Can't find a goal when both sides are variable")
                }
            }
        }
    }
}