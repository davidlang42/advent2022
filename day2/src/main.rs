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
        let mut scores: Vec<i32> = text.split("\r\n").map(|s| calculate_score(s, false)
            .expect(&format!("Error calculating score of {}", s))).collect();
        println!("Part 1 total score: {}", scores.iter().sum::<i32>());
        scores = text.split("\r\n").map(|s| calculate_score(s, true)
            .expect(&format!("Error calculating score of {}", s))).collect();
        println!("Part 2 total score: {}", scores.iter().sum::<i32>());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn calculate_score(text: &str, process_as_need: bool) -> Result<i32, String> {
    let hands: Vec<HandChoice> = text.split(" ").map(|s| parse_hand(s).expect(&format!("Hand could not be parsed: {}", s))).collect();
    if hands.len() != 2 {
        return Err(format!("More than 2 hands found in: {}", text));
    }
    let them = hands[0];
    let mut us = hands[1];
    if process_as_need {
        us = process_need(them, us);
    }
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
        'X' => Ok(HandChoice::Rock), // need lose
        'Y' => Ok(HandChoice::Paper), // need draw
        'Z' => Ok(HandChoice::Scissors), // need win
        _ => Err(format!("Incorrect hand choice: {}", c))
    }
}

// yes I know this is awful but oh well
fn process_need(them: HandChoice, need: HandChoice) -> HandChoice {
    match (them, need) {
        // draw
        (a, HandChoice::Paper) => a,
        // win
        (HandChoice::Rock, HandChoice::Scissors) => HandChoice::Paper,
        (HandChoice::Paper, HandChoice::Scissors) => HandChoice::Scissors,
        (HandChoice::Scissors, HandChoice::Scissors) => HandChoice::Rock,
        // lose
        (HandChoice::Rock, HandChoice::Rock) => HandChoice::Scissors,
        (HandChoice::Paper, HandChoice::Rock) => HandChoice::Rock,
        (HandChoice::Scissors, HandChoice::Rock) => HandChoice::Paper,
    }
}