use std::env;
use std::fs;

#[derive(Eq, PartialEq, Copy, Clone)]
enum HandChoice {
    Rock,
    Paper,
    Scissors
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let scores: Vec<i32> = text.split("\r\n").map(|s| calculate_score(s)
            .expect(&format!("Error calculating score of {}", s))).collect();
        println!("Total score: {}", scores.iter().sum::<i32>());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn calculate_score(text: &str) -> Result<i32, String> {
    let hands: Vec<HandChoice> = text.split(" ").map(|s| parse_hand(s).expect(&format!("Hand could not be parsed: {}", s))).collect();
    if hands.len() != 2 {
        return Err(format!("More than 2 hands found in: {}", text));
    }
    let them = hands[0];
    let us = hands[1];
    let shape = match us {
        HandChoice::Rock => 1,
        HandChoice::Paper => 2,
        HandChoice::Scissors => 3
    };
    let win = match (them, us) {
        (a, b) if a == b => 3, // draw
        (HandChoice::Rock, HandChoice::Paper) => 6, // win
        (HandChoice::Paper, HandChoice::Scissors) => 6, // win
        (HandChoice::Scissors, HandChoice::Rock) => 6, // win
        _ => 0 // lose
    };
    Ok(shape+win)
}

fn parse_hand(hand: &str) -> Result<HandChoice, String> {
    if hand.len() != 1 {
        return Err(format!("Hand contained more than 1 character: {}", hand))
    }
    let c: char = hand[0..1].parse().expect(&format!("Failed to parse character: {}", hand));
    match c {
        'A' => Ok(HandChoice::Rock),
        'B' => Ok(HandChoice::Paper),
        'C' => Ok(HandChoice::Scissors),
        'X' => Ok(HandChoice::Rock),
        'Y' => Ok(HandChoice::Paper),
        'Z' => Ok(HandChoice::Scissors),
        _ => Err(format!("Incorrect hand choice: {}", c))
    }
}