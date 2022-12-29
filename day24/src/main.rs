use std::env;
use std::fs;
use pathfinding::prelude::bfs;
use std::str::FromStr;
use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Position {
    row: usize,
    col: usize
}

struct Grid {
    width: usize,
    height: usize,
    start: Position,
    finish: Position,
    tiles: Vec<Vec<Tile>>,
    blizzards: Vec<Blizzard>
}

struct Blizzard {
    initial_position: Position,
    direction: Direction
}

enum Tile {
    Wall,
    Open
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct State {
    position: Position,
    minute: usize
}

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

const NL: &str = "\n";

impl Blizzard {
    fn current_position(&self, minute: usize, grid: &Grid) -> Position {
        let mut col: isize = match self.direction {
            Direction::Right => self.initial_position.col as isize + minute as isize,
            Direction::Left => self.initial_position.col as isize - minute as isize,
            _ => self.initial_position.col as isize
        };
        let mut row: isize = match self.direction {
            Direction::Up => self.initial_position.row as isize - minute as isize,
            Direction::Down => self.initial_position.row as isize + minute as isize,
            _ => self.initial_position.row as isize
        };
        col = (col - 1).rem_euclid(grid.width as isize - 2) + 1;
        row = (row - 1).rem_euclid(grid.height as isize - 2) + 1;
        Position {
            row: row as usize,
            col: col as usize
        }
    }
}

impl State {
    fn successors(&self, grid: &Grid) -> Vec<Self> {
        let mut options = Vec::new();
        let mut state = self.clone();
        state.minute += 1;
        let blizzards: HashSet<Position> = grid.blizzards.iter().map(|b| b.current_position(state.minute, grid)).collect();
        for p in self.position.connections(grid.width, grid.height) {
            match grid.tiles[p.row][p.col] {
                Tile::Open => {
                    if !blizzards.contains(&p) {
                        let mut adjacent_state = state.clone();
                        adjacent_state.position = p;
                        options.push(adjacent_state);
                    }
                },
                _ => {}
            }
        }
        if !blizzards.contains(&state.position) {
            options.push(state);
        }
        options
    }
}

impl Position {
    fn connections(&self, width: usize, height: usize) -> Vec<Position> {
        let mut connections: Vec<Position> = Vec::new();
        if self.row > 0 {
            // up
            connections.push(Position {
                row: self.row - 1,
                col: self.col
            });
        }
        if self.col > 0 {
            // left
            connections.push(Position {
                row: self.row,
                col: self.col - 1
            });
        }
        if self.row < height - 1 {
            // down
            connections.push(Position {
                row: self.row + 1,
                col: self.col
            });
        }
        if self.col < width - 1 {
            // right
            connections.push(Position {
                row: self.row,
                col: self.col + 1
            });
        }
        connections
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut tiles = Vec::new();
        let mut blizzards = Vec::new();
        for (r, line) in text.split(NL).enumerate() {
            let mut row = Vec::new();
            for (c, ch) in line.chars().enumerate() {
                row.push(match ch {
                    '#' => Tile::Wall,
                    _ => Tile::Open
                });
                if let Some(direction) = match ch {
                    '^' => Some(Direction::Up),
                    'v' => Some(Direction::Down),
                    '>' => Some(Direction::Right),
                    '<' => Some(Direction::Left),
                    _ => None
                } {
                    blizzards.push(Blizzard {
                        initial_position: Position {
                            row: r,
                            col: c
                        },
                        direction
                    });
                }
            }
            tiles.push(row);
        }
        Ok(Grid {
            height: tiles.len(),
            width: tiles[0].len(),
            start: first_open(&tiles, 0),
            finish: first_open(&tiles, tiles.len() - 1),
            tiles,
            blizzards
        })
    }
}

fn first_open(tiles: &Vec<Vec<Tile>>, row: usize) -> Position {
    for (col, tile) in tiles[row].iter().enumerate() {
        match tile {
            Tile::Open => return Position { row, col },
            _ => { }
        }
    }
    panic!("Open tile not found.");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let grid: Grid = text.parse().unwrap();
        println!("{}x{} grid {} blizzards", grid.width, grid.height, grid.blizzards.len());
        let mut state = State {
            position: grid.start,
            minute: 0
        };
        let mut path = bfs(
            &state,
            |s| s.successors(&grid),
            |s| s.position == grid.finish
        ).unwrap();
        state = path.last().unwrap().clone();
        println!("Minutes there: {}", state.minute);
        path = bfs(
            &state,
            |s| s.successors(&grid),
            |s| s.position == grid.start
        ).unwrap();
        state = path.last().unwrap().clone();
        println!("Minutes back: {}", state.minute);
        path = bfs(
            &state,
            |s| s.successors(&grid),
            |s| s.position == grid.finish
        ).unwrap();
        state = path.last().unwrap().clone();
        println!("Minutes there again: {}", state.minute);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}