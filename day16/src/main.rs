use std::env;
use std::fs;
use std::collections::HashMap;

struct Valve<'a> {
    name: String,
    rate: usize,
    tunnels: Vec<&'a Valve<'a>>
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
            let mut valve = match valves.get_mut(&name) {
                Some(existing) => existing,
                None => {
                    let mut new = Valve { name: name.clone(), rate: 0, tunnels: Vec::new() };
                    valves.insert(name.clone(), new);
                    valves.get_mut(&name).unwrap()
                }
            };
            valve.rate = rate;
            let tunnels: Vec<&str> = sections[1].split(" ").skip(4).collect();
            for tunnel in tunnels {
                let tunnel_name = tunnel[0..2].to_string();
                let tunnel_valve = match valves.get(&tunnel_name) {
                    Some(existing) => existing,
                    None => {
                        let mut new = Valve { name: tunnel_name.clone(), rate: 0, tunnels: Vec::new() };
                        valves.insert(tunnel_name.clone(), new);
                        valves.get(&tunnel_name).unwrap()
                    }
                };
                valve.tunnels.push(&tunnel_valve);
            }
        }
        
        // let path_from_a = bfs(
        //     &finish,
        //     |p| p.successors(&grid, &max),
        //     |p| grid[p.row][p.col] == 'a' as u32
        // ).unwrap();
        println!("Valves: {}", valves.len());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}