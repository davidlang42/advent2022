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
        let visible = count_visible(&grid, Direction::FromLeft)
            + count_visible(&grid, Direction::FromRight)
            + count_visible(&grid, Direction::FromTop)
            + count_visible(&grid, Direction::FromBottom);
        println!("Total visible: {}", visible);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn count_visible(grid: &Vec<Vec<u32>>, direction: Direction) -> usize {
    let mut previous: u32;
    let (swap, r_range, c_range) = match direction {
        Direction::FromLeft => (false, 0..grid.len(), 0..grid[0].len()),
        Direction::FromTop => (true, 0..grid[0].len(), 0..grid.len()),
        Direction::FromRight => (false, 0..grid.len(), grid[0].len()..0),
        Direction::FromBottom => (true, grid[0].len()..0, 0..grid.len()),
    };
    let mut count = 0;
    for r in r_range {
        previous = 0;
        for c in c_range.clone() {
            let value = if swap { grid[c][r] } else { grid[r][c] };
            if value > previous {
                count += 1;
                previous = value;
            } else {
                break;
            }
        }
    }
    count
}