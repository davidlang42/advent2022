use std::env;
use std::fs;
use pathfinding::prelude::bfs;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Position {
    row: usize,
    col: usize
}

const NL: &str = "\n";

impl Position {
    fn successors(&self, grid: &Vec<Vec<u32>>, max: &Position) -> Vec<Self> {
        connections(&max, self).into_iter().filter(|c| is_valid(&grid, self, c)).collect()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let start: Position = find_marker(&text, "S");
        let finish: Position = find_marker(&text, "E");
        let mut grid: Vec<Vec<u32>> = text.split(NL).map(|s| s.chars().map(|c| c as u32).collect()).collect();
        grid[start.row][start.col] = 'a' as u32;
        grid[finish.row][finish.col] = 'z' as u32;
        let max = Position { row: grid.len() - 1, col: grid[0].len() - 1 };
        let path = bfs(
            &start,
            |p| p.successors(&grid, &max),
            |p| *p == finish
        ).unwrap();
        println!("Shortest path: {}", path.len() - 1);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn find_marker(text: &str, marker: &str) -> Position {
    let lines: Vec<&str> = text.split(NL).collect();
    for (row, line) in lines.iter().enumerate() {
        if let Some(col) = line.find(marker) {
            return Position { row, col };
        }
    }
    panic!("Marker not found");
}

fn is_valid(grid: &Vec<Vec<u32>>, from: &Position, to: &Position) -> bool {
    let from_value = grid[from.row][from.col];
    let to_value = grid[to.row][to.col];
    to_value <= from_value + 1
}

fn connections(max: &Position, from: &Position) -> Vec<Position> {
    let mut connections: Vec<Position> = Vec::new();
    if from.row > 0 {
        // up
        connections.push(Position {
            row: from.row - 1,
            col: from.col
        });
    }
    if from.col > 0 {
        // left
        connections.push(Position {
            row: from.row,
            col: from.col - 1
        });
    }
    if from.row < max.row {
        // down
        connections.push(Position {
            row: from.row + 1,
            col: from.col
        });
    }
    if from.col < max.col {
        // right
        connections.push(Position {
            row: from.row,
            col: from.col + 1
        });
    }
    connections
}