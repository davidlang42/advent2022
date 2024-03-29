use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let numbers: Vec<isize> = text.split("\r\n").map(|s| s.parse::<isize>().unwrap()).collect();
        let (mut a, mut b, mut c) = run_mixing(&numbers, 1, 1);
        println!("(1 round, factor 1) 1000th: {}, 2000th: {}, 3000th: {}, SUM: {}", a, b, c, a+b+c);
        (a, b, c) = run_mixing(&numbers, 10, 811589153);
        println!("(10 rounds, factor 811589153) 1000th: {}, 2000th: {}, 3000th: {}, SUM: {}", a, b, c, a+b+c);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn move_position(positions: &mut Vec<usize>, index: usize, from: usize, to: usize) {
    //println!("Move from pos {} to pos {}", from, to);
    if from < to {
        // move forward
        for i in 0..positions.len() {
            if positions[i] > from && positions[i] <= to {
                positions[i] -= 1;
            }
        }
        positions[index] = to;
    } else if from > to {
        // move backwards
        for i in 0..positions.len() {
            if positions[i] < from && positions[i] >= to {
                positions[i] += 1;
            }
        }
        positions[index] = to;
    } else {
        // equal, no change
    }
}

fn get_by_position(positions: &Vec<usize>, numbers: &Vec<isize>, position: usize) -> isize {
    let actual_position = position % numbers.len();
    for i in 0..positions.len() {
        if positions[i] == actual_position {
            return numbers[i];
        }
    }
    panic!("Not found");
}

fn run_mixing(original_numbers: &Vec<isize>, rounds: usize, factor: isize) -> (isize, isize, isize) {
    let numbers: Vec<isize> = original_numbers.iter().map(|n| n * factor).collect();
    let mut positions: Vec<usize> = Vec::new();
    let mut zero_index = 0;
    for i in 0..numbers.len() {
        positions.push(i);
        if numbers[i] == 0 {
            zero_index = i;
        }
    }
    let loop_size = numbers.len() as isize - 1; // because being in the last place is the same as the first
    for _ in 0..rounds {
        for i in 0..numbers.len() {
            let start = positions[i];
            let mut finish = positions[i] as isize + numbers[i] % loop_size;
            while finish < 0 {
                finish += loop_size;
            }
            while finish >= numbers.len() as isize {
                finish -= loop_size;
            }
            move_position(&mut positions, i, start, finish as usize);
        }
    }
    let zero_position = positions[zero_index];
    let a = get_by_position(&positions, &numbers, zero_position + 1000);
    let b = get_by_position(&positions, &numbers, zero_position + 2000);
    let c = get_by_position(&positions, &numbers, zero_position + 3000);
    (a,b,c)
}