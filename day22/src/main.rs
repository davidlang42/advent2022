use std::env;
use std::fs;

struct Position {
    row: usize,
    column: usize,
    facing: Direction
}

#[derive(Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up
}

enum Instruction {
    Move(usize),
    TurnLeft,
    TurnRight
}

#[derive(PartialEq)]
enum Tile {
    None,
    Open,
    Blocked
}

enum WrapType {
    Flat,
    Cube
}

const NL: &str = "\r\n";
const NLNL: &str = "\r\n\r\n";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let sections: Vec<&str> = text.split(NLNL).collect();
        if sections.len() != 2 {
            panic!("Expected 2 sections");
        }
        let grid = parse_grid(sections[0]);
        let instructions = parse_instructions(sections[1]);
        println!("Grid: {}x{}", grid.len(), grid[0].len());
        println!("Instructions: {}", instructions.len());
        //part1
        let mut pos = find_starting_position(&grid);
        for instruction in instructions {
            instruction.process(&mut pos, &grid, WrapType::Flat);
        }
        println!("Final position: {},{} facing {:?}", pos.row, pos.column, pos.facing);
        println!("Password: {}", 1000 * (pos.row + 1) + 4 * (pos.column + 1) + pos.facing.value());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn parse_grid(text: &str) -> Vec<Vec<Tile>> {//TODO handle max line length
    let mut grid = Vec::new();
    let mut max_length = 0;
    for line in text.split(NL) {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(match c {
                '.' => Tile::Open,
                '#' => Tile::Blocked,
                ' ' => Tile::None,
                _ => panic!("Invalid tile: {}", c)
            });
        }
        if row.len() > max_length {
            max_length = row.len();
        }
        grid.push(row);
    }
    for row in &mut grid {
        while row.len() < max_length {
            row.push(Tile::None);
        }
    }
    grid
}

fn parse_instructions(line: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let mut current_number: Vec<char> = Vec::new();
    for c in line.chars() {
        match c {
            'L' => {
                instructions.push(Instruction::Move(current_number.iter().collect::<String>().parse().unwrap()));
                current_number = Vec::new();
                instructions.push(Instruction::TurnLeft);
            },
            'R' => {
                instructions.push(Instruction::Move(current_number.iter().collect::<String>().parse().unwrap()));
                current_number = Vec::new();
                instructions.push(Instruction::TurnRight);
            },
            _ => current_number.push(c)
        }
    }
    if current_number.len() > 0 {
        instructions.push(Instruction::Move(current_number.iter().collect::<String>().parse().unwrap()));
    }
    instructions
}

fn find_starting_position(grid: &Vec<Vec<Tile>>) -> Position {
    let mut pos = Position { row: 0, column: 0, facing: Direction::Right };
    while grid[pos.row][pos.column] != Tile::Open {
        pos.column += 1;
    }
    pos
}

impl Direction {
    fn value(&self) -> usize {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3
        }
    }

    fn left(&self) -> Self {
        match self {
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left
        }
    }

    fn right(&self) -> Self {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right
        }
    }

    fn move_delta(&self) -> (isize, isize) { // row delta, column delta
        match self {
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Up => (-1, 0)
        }
    }
}

impl Instruction {
    fn process(&self, position: &mut Position, grid: &Vec<Vec<Tile>>, wrap_type: WrapType) {
        match self {
            Instruction::TurnLeft => position.facing = position.facing.left(),
            Instruction::TurnRight => position.facing = position.facing.right(),
            Instruction::Move(max) => {
                let (dr, dc) = position.facing.move_delta();
                for _ in 0..*max {
                    let (new_r, new_c) = wrap_type.wrap_position(&position, &grid, dr, dc);
                    if grid[new_r][new_c] == Tile::Open {
                        position.row = new_r;
                        position.column = new_c;
                    } else { // blocked
                        break;
                    }
                }
            }
        }
    }
}

impl WrapType {
    fn wrap_position(&self, position: &Position, grid: &Vec<Vec<Tile>>, dr: isize, dc: isize) -> (usize, usize) {
        let mut r = position.row as isize + dr;
        let mut c = position.column as isize + dc;
        loop {
            if r < 0 {
                r = (grid.len() - 1) as isize;
            } else if c < 0 {
                c = (grid[0].len() - 1) as isize;
            } else if r as usize == grid.len() {
                r = 0;
            } else if c as usize == grid[0].len() {
                c = 0;
            } else if grid[r as usize][c as usize] == Tile::None {
                r += dr;
                c += dc;
            } else {
                break;
            }
        }
        (r as usize, c as usize)
    }
}