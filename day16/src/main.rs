use std::env;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use pathfinding::prelude::bfs;

struct Valve {
    name: String,
    rate: usize,
    tunnels: Vec<String>
}

const NL: &str = "\n";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut valves: HashMap<String, Valve> = HashMap::new();
        for line in text.split(NL) { //Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
            let sections: Vec<&str> = line.split("; ").collect();
            let first: Vec<&str> = sections[0].split(" ").collect();
            let name = first[1].to_string();
            let rate = first[4].split("=").skip(1).next().unwrap().parse().unwrap();
            let tunnels: Vec<String> = sections[1].split(" ").skip(4).map(|s| s[0..2].to_string()).collect();
            let valve = Valve { name: name.clone(), rate, tunnels };
            valves.insert(name, valve);
        }
        println!("Valves: {}", valves.len());
        let useful: HashSet<String> = valves.values().filter(|v| v.rate > 0).map(|v| v.name.clone()).collect();
        println!("Useful: {}", useful.len());
        let best = best_simulation(&valves, "AA", &useful, 30);
        println!("Best: {}", best);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn best_simulation(valves: &HashMap<String, Valve>, current: &str, unopened: &HashSet<String>, remaining: usize) -> usize {
    if remaining == 0 || unopened.len() == 0 {
        return 0;
    }
    let mut best_option = 0;
    for next_key in unopened {
        let next_valve = valves.get(next_key).unwrap();
        let time_to_move_and_open = shortest_distance(valves, current, next_key) + 1;
        let this_option: usize;
        if time_to_move_and_open > remaining {
            this_option = 0;
        } else {
            let mut next_unopened = unopened.clone();
            next_unopened.remove(next_key);
            let next_remaining = remaining - time_to_move_and_open;
            this_option = next_remaining * next_valve.rate + best_simulation(valves, next_key, &next_unopened, next_remaining);
        }
        if this_option > best_option {
            best_option = this_option;
        }
    }
    best_option
}

fn shortest_distance(valves: &HashMap<String, Valve>, from: &str, to: &str) -> usize {
    bfs(&from.to_string(), |name| valves.get(name).unwrap().tunnels.clone(), |p| *p == *to).unwrap().len() - 1
}

// fn best_case(valves: &HashMap<String, Valve>, unopened: &HashSet<String>, minutes: usize) -> usize {
//     if minutes < 2 {
//         return 0;
//     }
//     let mut remaining = minutes;
//     let mut sum = 0;
//     let mut unopened_rates: Vec<usize> = unopened.iter().map(|u| valves.get(u).unwrap().rate).collect();
//     unopened_rates.sort();
//     for r in unopened_rates.iter().rev() {
//         remaining -= 2; // best case is 1min to move then 1min to open
//         sum += r * remaining;
//         if remaining < 2 {
//             break;
//         }
//     }
//     sum
// }