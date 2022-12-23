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
        let mut grid = parse_grid(sections[0]);
        let instructions = parse_instructions(sections[1]);
        println!("Grid: {}x{}", grid.len(), grid[0].len());
        println!("Instructions: {}", instructions.len());
        //part1
        let mut pos = find_starting_position(&grid);
        for instruction in &instructions {
            instruction.process(&mut pos, &grid, WrapType::Flat);
        }
        println!("Part1 Final position: {},{} facing {:?}", pos.row, pos.column, pos.facing);
        println!("Part1 Password: {}", 1000 * (pos.row + 1) + 4 * (pos.column + 1) + pos.facing.value());
        //part2
        let mut changed = false;
        if grid.len() > grid[0].len() {
            grid = change_grid(&grid);
            changed = true;
        }
        for r in 0..grid.len() {
            let mut line = Vec::new();
            for c in 0..grid[0].len() {
                line.push(match grid[r][c] {
                    Tile::None => ' ',
                    Tile::Open => '.',
                    Tile::Blocked => '#'
                });
            }
            println!("{}", line.iter().collect::<String>());
        }
        let mut pos = find_starting_position(&grid);
        for instruction in &instructions {
            instruction.process(&mut pos, &grid, WrapType::Cube);
        }
        if changed {
            change_position(&mut pos, grid.len()/3);
        }
        println!("Part2 Final position: {},{} facing {:?}", pos.row, pos.column, pos.facing);
        println!("Part2 Password: {}", 1000 * (pos.row + 1) + 4 * (pos.column + 1) + pos.facing.value());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn change_grid(old: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    // change grid from:
    //  16
    //  4
    // 35
    // 2
    // to:
    //   1
    // 234
    //   56
    // (because Eric Wastl is a dick)
    let cube = old.len() / 4;
    if cube != old[0].len() / 3 {
        panic!("Not a convertable cube");
    }
    let mut new = Vec::new();
    for r in 0..(3*cube) {
        let mut row = Vec::new();
        for _ in 0..(2*cube) {
            row.push(Tile::None);
        }
        for c in (2*cube)..(3*cube) {
            row.push(old[r][c-cube]);
        }
        for _ in (3*cube)..(4*cube) {
            row.push(Tile::None);
        }
        new.push(row);
    }
    let mut face3 = extract_square(old, 2*cube, 0, cube);
    let mut face2 = extract_square(old, 3*cube, 0, cube);
    let mut face6 = extract_square(old, 0, 2*cube, cube);
    face2 = rotate_square_clockwise(&face2);
    face3 = rotate_square_clockwise(&face3);
    face6 = rotate_square_clockwise(&rotate_square_clockwise(&face6));
    insert_square(&mut new, cube, 0, &face2);
    insert_square(&mut new, cube, cube, &face3);
    insert_square(&mut new, 2*cube, 3*cube, &face6);
    new
}

fn change_position(pos: &mut Position, cube: usize) {
    if pos.column >= 2*cube && pos.column < 3*cube {
        // 1,4,5
        pos.column += cube;
    } else if pos.row < 2*cube {
        // 2,3
        (pos.row, pos.column) = (4*cube - pos.column - 1, pos.row - cube);
        pos.facing = pos.facing.left();
    } else {
        // 6
        (pos.row, pos.column) = (3*cube - pos.row - 1, 6*cube - pos.column - 1);
        pos.facing = pos.facing.left().left();
    }
}

fn extract_square(old: &Vec<Vec<Tile>>, start_r: usize, start_c: usize, length: usize) -> Vec<Vec<Tile>> {
    let mut new = Vec::new();
    for r in start_r..(start_r + length) {
        let mut row = Vec::new();
        for c in start_c..(start_c + length) {
            row.push(old[r][c]);
        }
        new.push(row);
    }
    new
}

fn rotate_square_clockwise(old: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let mut new = Vec::new();
    for c in 0..old[0].len() {
        let mut row = Vec::new();
        let mut r = old.len();
        while r > 0 {
            r -= 1;
            row.push(old[r][c]);
        }
        new.push(row);
    }
    new
}

fn insert_square(grid: &mut Vec<Vec<Tile>>, start_r: usize, start_c: usize, insert: &Vec<Vec<Tile>>) {
    for r in 0..insert.len() {
        for c in 0..insert[0].len() {
            grid[start_r+r][start_c+c] = insert[r][c];
        }
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
            }
        }
        Position { row: r as usize, column: c as usize, facing: f }
    }
}