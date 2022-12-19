use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Point {
    x: isize,
    y: isize
}

struct Blueprint {
    ore_per_ore_robot: usize,
    ore_per_clay_robot: usize,
    ore_per_obsidian_robot: usize,
    clay_per_obsidian_robot: usize,
    ore_per_geode_robot: usize,
    obsidian_per_geode_robot: usize
}

#[derive(Copy, Clone)]
struct State {
    minutes_remaining: usize,
    ore: usize,
    ore_robots: usize,
    clay: usize,
    clay_robots: usize,
    obsidian: usize,
    obsidian_robots: usize,
    geodes: usize,
    geode_robots: usize
}

enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 4 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let blueprints: Vec<Blueprint> = text.split("\r\n").map(|s| s.parse().unwrap()).collect();
        let minutes: usize = args[3].parse().unwrap();
        if let Ok(bp_index) = args[2].parse::<usize>() {
            println!("Starting Blueprint {} for {} minutes", bp_index, minutes);
            let final_state = max_geodes(&blueprints[bp_index-1], State::new(minutes));
            let quality = bp_index * final_state.geodes;
            println!("Blueprint {} makes {} geodes with a quality of {}", bp_index, final_state.geodes, quality);
        } else {
            let mut sum = 0;
            for (i, bp) in blueprints.iter().enumerate() {
                println!("Starting Blueprint {} for {} minutes", i+1, minutes);
                let final_state = max_geodes(bp, State::new(minutes));
                let quality = (i+1) * final_state.geodes;
                sum += quality;
                println!("Blueprint {} makes {} geodes with a quality of {}", i+1, final_state.geodes, quality);
            }
            println!("Sum: {}", sum);
        }
    } else {
        println!("Please provide 2 arguments: Filename, Blueprint number, Minutes");
    }
}

impl FromStr for Blueprint {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        //Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
        let words: Vec<&str> = line.split(" ").collect();
        if words.len() != 32 {
            panic!("Should have 32 words")
        } else {
            Ok(Blueprint {
                ore_per_ore_robot: words[6].parse().unwrap(),
                ore_per_clay_robot: words[12].parse().unwrap(),
                ore_per_obsidian_robot: words[18].parse().unwrap(),
                clay_per_obsidian_robot: words[21].parse().unwrap(),
                ore_per_geode_robot: words[27].parse().unwrap(),
                obsidian_per_geode_robot: words[30].parse().unwrap()
            })
        }
    }
}

impl State {
    fn new(minutes: usize) -> Self {
        State {
            minutes_remaining: minutes,
            ore: 0,
            ore_robots: 1,
            clay: 0,
            clay_robots: 0,
            obsidian: 0,
            obsidian_robots: 0,
            geodes: 0,
            geode_robots: 0
        }
    }
}

fn max_geodes(bp: &Blueprint, initial_state: State) -> State {
    let mut state = initial_state;
    let mut options: Vec<Option<Robot>> = vec![None];
    if state.ore >= bp.ore_per_geode_robot && state.obsidian >= bp.obsidian_per_geode_robot {
        options.push(Some(Robot::Geode));
    }
    if state.ore >= bp.ore_per_obsidian_robot && state.clay >= bp.clay_per_obsidian_robot {
        options.push(Some(Robot::Obsidian));
    }
    if state.ore >= bp.ore_per_clay_robot {
        options.push(Some(Robot::Clay));
    }
    if state.ore >= bp.ore_per_ore_robot {
        options.push(Some(Robot::Ore));
    }
    state.ore += state.ore_robots;
    state.clay += state.clay_robots;
    state.obsidian += state.obsidian_robots;
    state.geodes += state.geode_robots;
    state.minutes_remaining -= 1;
    //println!("{} min remaining: {} ore [{}], {} clay [{}], {} obsidian [{}], {} geodes [{}] => {} options", state.minutes_remaining, state.ore, state.ore_robots, state.clay, state.clay_robots, state.obsidian, state.obsidian_robots, state.geodes, state.geode_robots, options.len());
    if state.minutes_remaining == 0 {
        state
    } else {
        options.iter().map(|o| do_option(bp, o, state)).max_by(|a,b| a.geodes.cmp(&b.geodes)).unwrap()
    }
}

fn do_option(bp: &Blueprint, option: &Option<Robot>, existing_state: State) -> State {
    let mut state = existing_state;
    if let Some(new_robot) = option {
        match new_robot {
            Robot::Geode => {
                state.ore -= bp.ore_per_geode_robot;
                state.obsidian -= bp.obsidian_per_geode_robot;
                state.geode_robots += 1;
            },
            Robot::Obsidian => {
                state.ore -= bp.ore_per_obsidian_robot;
                state.clay -= bp.clay_per_obsidian_robot;
                state.obsidian_robots += 1;
            },
            Robot::Clay => {
                state.ore -= bp.ore_per_clay_robot;
                state.clay_robots += 1;
            },
            Robot::Ore => {
                state.ore -= bp.ore_per_ore_robot;
                state.ore_robots += 1;
            }
        }
    }
    max_geodes(bp, state)
}