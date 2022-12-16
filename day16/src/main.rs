use std::env;
use std::fs;
use std::collections::HashMap;

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
            let mut valve = Valve { name: name.clone(), rate, tunnels };
            valves.insert(name, valve);
        }
        
        println!("Valves: {}", valves.len());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}