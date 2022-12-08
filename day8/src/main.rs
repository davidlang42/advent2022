use std::env;
use std::fs;

const NL: &str = "\r\n";

enum Direction {
    FromLeft,
    FromRight,
    FromTop,
    FromBottom
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let grid: Vec<Vec<u32>> = text.split(NL).map(|row| row.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();
        println!("Rows: {}, Columns: {}", grid.len(), grid[0].len());
        let mut visible: Vec<Vec<bool>> = Vec::new();
        for _ in 0..grid.len() {
            let mut row = Vec::new();
            for _ in 0..grid[0].len() {
                row.push(false);
            }
            visible.push(row);
        }
        mark_visible(&mut visible, &grid, Direction::FromLeft);
        mark_visible(&mut visible, &grid, Direction::FromRight);
        mark_visible(&mut visible, &grid, Direction::FromTop);
        mark_visible(&mut visible, &grid, Direction::FromBottom);
        println!("Total visible: {}", count(&visible));
        let score = highest_scenic_score(&grid);
        println!("Highest score: {}", score);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn count(visible: &Vec<Vec<bool>>) -> usize {
    visible.iter().map(|row| row.iter().filter(|b| **b).count()).sum()
}

fn mark_visible(visible: &mut Vec<Vec<bool>>, grid: &Vec<Vec<u32>>, direction: Direction) {
    let (swap, r_range, c_range): (bool, Vec<usize>, Vec<usize>) = match direction {
        Direction::FromLeft => (false, (0..grid.len()).collect(), (0..grid[0].len()).collect()),
        Direction::FromTop => (true, (0..grid[0].len()).collect(), (0..grid.len()).collect()),
        Direction::FromRight => (false, (0..grid.len()).collect(), (0..grid[0].len()).rev().collect()),
        Direction::FromBottom => (true, (0..grid[0].len()).collect(), (0..grid.len()).rev().collect()),
    };
    for r in r_range {
        let mut previous = -1;
        for c in c_range.clone() {
            let value: isize = (if swap { grid[c][r] } else { grid[r][c] }).try_into().unwrap();
            if value > previous {
                if swap {
                    visible[c][r] = true;
                } else {
                    visible[r][c] = true;
                }
                previous = value;
            }
        }
    }
}

fn highest_scenic_score(grid: &Vec<Vec<u32>>) -> usize {
    let mut max = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            let value = find_scenic_score(grid, r, c);
            if value > max {
                max = value;
            }
        }
    }
    max
}

fn find_scenic_score(grid: &Vec<Vec<u32>>, r: usize, c: usize) -> usize {
    find_viewing_distance(grid, r, c, Direction::FromLeft)
        * find_viewing_distance(grid, r, c, Direction::FromRight)
        * find_viewing_distance(grid, r, c, Direction::FromTop)
        * find_viewing_distance(grid, r, c, Direction::FromBottom)
}

fn find_viewing_distance(grid: &Vec<Vec<u32>>, from_row: usize, from_col: usize, direction: Direction) -> usize {
    let (r_delta, c_delta): (isize, isize) = match direction {
        Direction::FromLeft => (-1, 0),
        Direction::FromRight => (1, 0),
        Direction::FromTop => (0, -1),
        Direction::FromBottom => (0, 1)
    };
    let mut distance = 0;
    let height = grid[from_row][from_col];
    let mut r: isize = from_row as isize + r_delta;
    let mut c: isize = from_col as isize + c_delta;
    let r_max: isize = grid.len().try_into().unwrap();
    let c_max: isize = grid[0].len().try_into().unwrap();
    while r >= 0 && r < r_max && c >= 0 && c < c_max {
        distance += 1;
        if grid[r as usize][c as usize] >= height {
            break;
        }
        r += r_delta;
        c += c_delta;
    }
    distance
}