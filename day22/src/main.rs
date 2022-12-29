use std::env;
use std::fs;

struct Position {
    row: usize,
    column: usize,
    facing: Direction
}

#[derive(Debug, Copy, Clone, PartialEq)]
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

#[derive(PartialEq, Copy, Clone)]
enum Tile {
    None,
    Open,
    Blocked
}

enum WrapType {
    Flat,
    Cube,
    Cube2
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
        for instruction in &instructions {
            instruction.process(&mut pos, &grid, &WrapType::Flat);
        }
        println!("Part1 Final position: {},{} facing {:?}", pos.row, pos.column, pos.facing);
        println!("Part1 Password: {}", 1000 * (pos.row + 1) + 4 * (pos.column + 1) + pos.facing.value());
        //part2
        let wrap_type = if grid.len() > grid[0].len() {
            WrapType::Cube2
        } else {
            WrapType::Cube
        };
        // for r in 0..grid.len() {
        //     let mut line = Vec::new();
        //     for c in 0..grid[0].len() {
        //         line.push(match grid[r][c] {
        //             Tile::None => ' ',
        //             Tile::Open => '.',
        //             Tile::Blocked => '#'
        //         });
        //     }
        //     println!("{}", line.iter().collect::<String>());
        // }
        let mut pos = find_starting_position(&grid);
        for instruction in &instructions {
            instruction.process(&mut pos, &grid, &wrap_type);
        }
        println!("Part2 Final position: {},{} facing {:?}", pos.row, pos.column, pos.facing);
        println!("Part2 Password: {}", 1000 * (pos.row + 1) + 4 * (pos.column + 1) + pos.facing.value());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn parse_grid(text: &str) -> Vec<Vec<Tile>> {
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
    fn process(&self, position: &mut Position, grid: &Vec<Vec<Tile>>, wrap_type: &WrapType) {
        match self {
            Instruction::TurnLeft => position.facing = position.facing.left(),
            Instruction::TurnRight => position.facing = position.facing.right(),
            Instruction::Move(max) => {
                for _ in 0..*max {
                    let new_pos = wrap_type.wrap_position(&position, &grid);
                    if grid[new_pos.row][new_pos.column] == Tile::Open {
                        *position = new_pos;
                    } else { // blocked
                        break;
                    }
                }
            }
        }
    }
}

impl WrapType {
    fn wrap_position(&self, position: &Position, grid: &Vec<Vec<Tile>>) -> Position {
        let (dr, dc) = position.facing.move_delta();
        let mut r = position.row as isize + dr;
        let mut c = position.column as isize + dc;
        let mut f = position.facing;
        let height = grid.len() as isize;
        let width = grid[0].len() as isize;
        match self {
            WrapType::Flat => {
                loop {
                    if r < 0 {
                        r = height - 1;
                    } else if c < 0 {
                        c = width - 1;
                    } else if r == height {
                        r = 0;
                    } else if c == width {
                        c = 0;
                    } else if grid[r as usize][c as usize] == Tile::None {
                        r += dr;
                        c += dc;
                    } else {
                        break;
                    }
                }
            },
            WrapType::Cube => {
                let cube = height / 3;
                if cube != width / 4 {
                    panic!("Not a cube");
                }
                loop {
                    if r < 0 && f == Direction::Up {
                        // 1 -> 2
                        r = cube;
                        c = 3 * cube - c - 1;
                        f = Direction::Down;
                    } else if c < 0 && f == Direction::Left {
                        // 2 -> 6
                        c = 5 * cube - r - 1;
                        r = 3 * cube - 1;
                        f = Direction::Up;
                    } else if r == height && f == Direction::Down {
                        if c < 3 * cube {
                            // 5 -> 2
                            r = 2 * cube - 1;
                            c = 3 * cube - c - 1;
                            f = Direction::Up;
                        } else {
                            // 6 -> 2
                            r = 5 * cube - c - 1;
                            c = 0;
                            f = Direction::Right;
                        }
                    } else if c == width && f == Direction::Right {
                        // 6 -> 1
                        r = 3 * cube - r - 1;
                        c = 3 * cube - 1;
                        f = Direction::Left;
                    } else if grid[r as usize][c as usize] == Tile::None {
                        match f {
                            Direction::Right => {
                                if r < cube {
                                    // 1 -> 6
                                    r = 3 * cube - r - 1;
                                    c = 4 * cube - 1;
                                    f = Direction::Left;
                                } else {
                                    // 4 -> 6
                                    c = 5 * cube - r - 1;
                                    r = 2 * cube;
                                    f = Direction::Down;
                                }
                            },
                            Direction::Down => {
                                if c < cube {
                                    // 2 -> 5
                                    c = 3 * cube - c - 1;
                                    r = 3 * cube - 1;
                                    f = Direction::Up;
                                } else {
                                    // 3 -> 5
                                    r = 4 * cube - c - 1;
                                    c = 2 * cube;
                                    f = Direction::Right;
                                }
                            },
                            Direction::Left => {
                                if r  < cube {
                                    // 1 -> 3
                                    c = r + cube;
                                    r = cube;
                                    f = Direction::Down;
                                } else {
                                    // 5 -> 3
                                    c = 4 * cube - r - 1;
                                    r = 2 * cube - 1;
                                    f = Direction::Up;
                                }
                            },
                            Direction::Up => {
                                if c < cube {
                                    // 2 -> 1
                                    c = 3 * cube - c - 1;
                                    r = 0;
                                    f = Direction::Down;
                                } else if c < 2 * cube {
                                    // 3 -> 1
                                    r = c - cube;
                                    c = 2 * cube;
                                    f = Direction::Right;
                                } else {
                                    // 6 -> 4
                                    r = 5 * cube - c - 1;
                                    c = 3 * cube - 1;
                                    f = Direction::Left;
                                }
                            }
                        }
                    } else {
                        break;
                    }
                }
            },
            WrapType::Cube2 => {
                let cube = height / 4;
                if cube != width / 3 {
                    panic!("Not a cube2");
                }
                loop {
                    if r < 0 && f == Direction::Up {
                        if c < 2 * cube {
                            // 1 -> 2
                            r = c + 2 * cube;
                            c = 0;
                            f = Direction::Right;
                        } else {
                            // 6 -> 2
                            r = 4 * cube - 1;
                            c = c - 2 * cube;
                            f = Direction::Up;
                        }
                    } else if c < 0 && f == Direction::Left {
                        if r < 3 * cube {
                            // 3 -> 1
                            r = 3 * cube - r - 1;
                            c = cube;
                            f = Direction::Right;
                        } else {
                            // 2 -> 1
                            c = r - 2 * cube;
                            r = 0;
                            f = Direction::Down;
                        }
                    } else if r == height && f == Direction::Down {
                        // 2 -> 6
                        c = c + 2 * cube;
                        r = 0;
                        f = Direction::Down;
                    } else if c == width && f == Direction::Right {
                        // 6 -> 5
                        r = 3 * cube - r - 1;
                        c = 2 * cube - 1;
                        f = Direction::Left;
                    } else if grid[r as usize][c as usize] == Tile::None {
                        match f {
                            Direction::Right => {
                                if r < 2 * cube {
                                    // 4 -> 6
                                    c = r + cube;
                                    r = cube - 1;
                                    f = Direction::Up;
                                } else if r < 3 * cube {
                                    // 5 -> 6
                                    r = 3 * cube - 1 - r;
                                    c = 3 * cube - 1;
                                    f = Direction::Left;
                                } else {
                                    // 2 -> 5
                                    c = r - 2 * cube;
                                    r = 3 * cube - 1;
                                    f = Direction::Up;
                                }
                            },
                            Direction::Down => {
                                if c < 2 * cube {
                                    // 5 -> 2
                                    r = 2 * cube + c;
                                    c = cube - 1;
                                    f = Direction::Left;
                                } else {
                                    // 6 -> 4
                                    r = c - cube;
                                    c = 2 * cube - 1;
                                    f = Direction::Left;
                                }
                            },
                            Direction::Left => {
                                if r < cube {
                                    // 1 -> 3
                                    c = 0;
                                    r = 3 * cube - 1 - r;
                                    f = Direction::Right;
                                } else {
                                    // 4 -> 3
                                    c = r - cube;
                                    r = 2 * cube;
                                    f = Direction::Down;
                                }
                            },
                            Direction::Up => {
                                // 3 -> 4
                                r = c + cube;
                                c = cube;
                                f = Direction::Right;
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        Position { row: r as usize, column: c as usize, facing: f }
    }
}