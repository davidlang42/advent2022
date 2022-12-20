use std::env;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use pathfinding::prelude::bfs;
use itertools::Itertools;

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
        let mut cache: HashMap<(String, String),usize> = HashMap::new();
        let best = best_simulation(&valves, "AA", &useful, 30, &mut cache);
        println!("Part1 Best: {}", best);
        let mut best_total = 0;
        for half in useful.clone().into_iter().combinations(useful.len() / 2) {
            let me = half.into_iter().collect();
            let elephant: HashSet<String> = useful.difference(&me).map(|s| s.to_string()).collect();
            let total = best_simulation(&valves, "AA", &me, 26, &mut cache) + best_simulation(&valves, "AA", &elephant, 26, &mut cache);
            if total > best_total {
                best_total = total;
            }
        }
        println!("Part2 Best: {}", best_total);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn best_simulation(valves: &HashMap<String, Valve>, current: &str, unopened: &HashSet<String>, remaining: usize, cache: &mut HashMap<(String, String),usize>) -> usize {
    if remaining == 0 || unopened.len() == 0 {
        return 0;
    }
    let mut best_option = 0;
    for next_key in unopened {
        let next_valve = valves.get(next_key).unwrap();
        let time_to_move_and_open = shortest_distance(valves, current, next_key, cache) + 1;
        let this_option: usize;
        if time_to_move_and_open > remaining {
            this_option = 0;
        } else {
            let mut next_unopened = unopened.clone();
            next_unopened.remove(next_key);
            let next_remaining = remaining - time_to_move_and_open;
            let this_release = next_remaining * next_valve.rate;
            if this_release + best_case(valves, &next_unopened, next_remaining) <= best_option {
                this_option = 0; // don't bother
            } else {
                this_option = this_release + best_simulation(valves, next_key, &next_unopened, next_remaining, cache);
            }
        }
        if this_option > best_option {
            best_option = this_option;
        }
    }
    best_option
}

fn shortest_distance(valves: &HashMap<String, Valve>, from: &str, to: &str, cache: &mut HashMap<(String, String),usize>) -> usize {
    let key = (from.to_string(), to.to_string());
    if let Some(previous) = cache.get(&key) {
        *previous
    } else {
        let result = bfs(&key.0, |name| valves.get(name).unwrap().tunnels.clone(), |p| *p == *to).unwrap().len() - 1;
        cache.insert(key, result);
        result
    }
}

fn best_case(valves: &HashMap<String, Valve>, unopened: &HashSet<String>, minutes: usize) -> usize {
    if minutes < 2 {
        return 0;
    }
    let mut remaining = minutes;
    let mut sum = 0;
    let mut unopened_rates: Vec<usize> = unopened.iter().map(|u| valves.get(u).unwrap().rate).collect();
    unopened_rates.sort();
    for r in unopened_rates.iter().rev() {
        remaining -= 2; // best case is 1min to move then 1min to open
        sum += r * remaining;
        if remaining < 2 {
            break;
        }
    }
    sum
}