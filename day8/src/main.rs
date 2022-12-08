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