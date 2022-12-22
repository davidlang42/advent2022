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
        // expressions.insert("root".to_string(), match &expressions["root"] {
        //     Expression::Literal(_) => panic!("Root should have been an operation"),
        //     Expression::Product(a, b) => Expression::Difference(a.to_string(), b.to_string()),
        //     Expression::Sum(a, b) => Expression::Difference(a.to_string(), b.to_string()),
        //     Expression::Difference(a, b) => Expression::Difference(a.to_string(), b.to_string()),
        //     Expression::Division(a, b) => Expression::Difference(a.to_string(), b.to_string())
        // });
        // println!("My value: {}", expressions["root"].goal_find("root", 0, &expressions, "humn"));
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

impl Assignment {
    fn calculate(&self, assignments: &HashMap<String, Assignment>) -> isize {
        match &self.expression {
            Expression::Literal(l) => *l,
            Expression::Algebra(a, op, b) => {
                match op {
                    Operation::Product => assignments[a].calculate(assignments) * assignments[b].calculate(assignments),
                    Operation::Sum => assignments[a].calculate(assignments) + assignments[b].calculate(assignments),
                    Operation::Difference => assignments[a].calculate(assignments) - assignments[b].calculate(assignments),
                    Operation::Division => assignments[a].calculate(assignments) / assignments[b].calculate(assignments)
                }
            }
        }
    }

    // fn calculate_without(&self, other_expressions: &HashMap<String, Expression>, excluding: &str) -> Option<isize> {
    //     match self {
    //         Expression::Literal(l) => Some(*l),
    //         Expression::Product(a, b) => {
    //             println!("Calc {},{} without {}", a, b, excluding);
    //             if excluding == *a || excluding == *b {
    //                 None
    //             } else {
    //                 Some(other_expressions[a].calculate_without(other_expressions, excluding)? * other_expressions[b].calculate_without(other_expressions, excluding)?)
    //             }
    //         },
    //         Expression::Sum(a, b) => {
    //             println!("Calc {},{} without {}", a, b, excluding);
    //             if excluding == *a || excluding == *b {
    //                 None
    //             } else {
    //                 Some(other_expressions[a].calculate_without(other_expressions, excluding)? + other_expressions[b].calculate_without(other_expressions, excluding)?)
    //             }
    //         },
    //         Expression::Difference(a, b) => {
    //             println!("Calc {},{} without {}", a, b, excluding);
    //             if excluding == *a || excluding == *b {
    //                 None
    //             } else {
    //                 Some(other_expressions[a].calculate_without(other_expressions, excluding)? - other_expressions[b].calculate_without(other_expressions, excluding)?)
    //             }
    //         },
    //         Expression::Division(a, b) => {
    //             println!("Calc {},{} without {}", a, b, excluding);
    //             if excluding == *a || excluding == *b {
    //                 None
    //             } else {
    //                 Some(other_expressions[a].calculate_without(other_expressions, excluding)? / other_expressions[b].calculate_without(other_expressions, excluding)?)
    //             }
    //         },
    //     }
    // }

    // fn goal_find(&self, my_name: &str, goal: isize, expressions: &HashMap<String, Expression>, find_value_of: &str) -> isize {
    //     if *my_name == *find_value_of {
    //         return goal;
    //     }
    //     match self {
    //         Expression::Literal(l) => *l,
    //         Expression::Product(a, b) => {
    //             let a_value = expressions[a].calculate_without(expressions, find_value_of);
    //             let b_value = expressions[b].calculate_without(expressions, find_value_of);
    //             match (a_value, b_value) {
    //                 (Some(_), Some(_)) => panic!("Can't find a goal when both sides are literal: {}, {}", a, b),
    //                 (Some(a_literal), None) => expressions[b].goal_find(b, goal / a_literal, expressions, find_value_of),
    //                 (None, Some(b_literal)) => expressions[a].goal_find(a, goal / b_literal, expressions, find_value_of),
    //                 (None, None) => panic!("Can't find a goal when both sides are variable: {}, {}", a, b)
    //             }
    //         },
    //         Expression::Sum(a, b) => {
    //             let a_value = expressions[a].calculate_without(expressions, find_value_of);
    //             let b_value = expressions[b].calculate_without(expressions, find_value_of);
    //             match (a_value, b_value) {
    //                 (Some(_), Some(_)) => panic!("Can't find a goal when both sides are literal: {}, {}", a, b),
    //                 (Some(a_literal), None) => expressions[b].goal_find(b, goal - a_literal, expressions, find_value_of),
    //                 (None, Some(b_literal)) => expressions[a].goal_find(a, goal - b_literal, expressions, find_value_of),
    //                 (None, None) => panic!("Can't find a goal when both sides are variable: {}, {}", a, b)
    //             }
    //         },
    //         Expression::Difference(a, b) => {
    //             let a_value = expressions[a].calculate_without(expressions, find_value_of);
    //             let b_value = expressions[b].calculate_without(expressions, find_value_of);
    //             match (a_value, b_value) {
    //                 (Some(_), Some(_)) => panic!("Can't find a goal when both sides are literal: {}, {}", a, b),
    //                 (Some(a_literal), None) => expressions[b].goal_find(b, a_literal - goal, expressions, find_value_of),
    //                 (None, Some(b_literal)) => expressions[a].goal_find(a, goal + b_literal, expressions, find_value_of),
    //                 (None, None) => panic!("Can't find a goal when both sides are variable: {}, {}", a, b)
    //             }
    //         },
    //         Expression::Division(a, b) => {
    //             let a_value = expressions[a].calculate_without(expressions, find_value_of);
    //             let b_value = expressions[b].calculate_without(expressions, find_value_of);
    //             match (a_value, b_value) {
    //                 (Some(_), Some(_)) => panic!("Can't find a goal when both sides are literal: {}, {}", a, b),
    //                 (Some(a_literal), None) => expressions[b].goal_find(b, goal * a_literal, expressions, find_value_of),
    //                 (None, Some(b_literal)) => expressions[a].goal_find(a, goal * b_literal, expressions, find_value_of),
    //                 (None, None) => panic!("Can't find a goal when both sides are variable: {}, {}", a, b)
    //             }
    //         }
    //     }
    // }
}