use std::env;
use std::fs;
use std::str::FromStr;
use std::collections::VecDeque;

const NL: &str = "\n";

struct File {
    name: String,
    size: usize
}

struct Directory {
    name: String,
    files: Vec<File>,
    directories: Vec<Directory>,
}

fn get_dir_size(dir: &Directory) -> usize {
    dir.files.iter().map(|f| f.size).sum::<usize>() + dir.directories.iter().map(|d| get_dir_size(d)).sum::<usize>()
}

enum Command {
    ChangeDir(String),
    ChangeRoot,
    ChangeBack,
    List(Vec<String>)
}

impl FromStr for Command {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        if text.starts_with("cd") {
            let dir = text[3..].split(NL).next().unwrap();
            if dir.eq("..") {
                Ok(Command::ChangeBack)
            } else if dir.eq("/") {
                Ok(Command::ChangeRoot)
            } else {
                Ok(Command::ChangeDir(dir.to_owned()))
            }
        } else if text.starts_with("ls") {
            let lines = text.split(NL).skip(1).filter(|s| s.len() != 0).map(|s| s.to_owned()).collect();
            Ok(Command::List(lines))
        } else {
            Err(format!("Command not found: {}", text))
        }
    }
}

fn sum_dirs(pwd: &Directory, max_size: usize) -> usize {
    let mut sum = 0;
    let size = get_dir_size(pwd);
    if size <= max_size {
        sum += size;
    };
    for dir in &pwd.directories {
        sum += sum_dirs(&dir, max_size);
    }
    sum
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut commands: VecDeque<Command> = text.split("$ ").skip(1).map(|c| c.parse().unwrap()).collect();
        println!("Number of commands: {}", commands.len());
        let mut root = Directory { name: String::new(), files: Vec::new(), directories: Vec::new() };
        consume_commands(&mut root, &mut commands);
        println!("Root contains {} files and {} dirs", root.files.len(), root.directories.len());
        let sum = sum_dirs(&root, 100000);
        println!("Sum of directories up to max size: {}", sum);
        let free = 70000000 - get_dir_size(&root);
        let required = 30000000 - free;
        let delete = find_smallest_dir(&root, required);
        println!("Smallest dir to provide enough space: {}", delete);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn consume_commands(pwd: &mut Directory, commands: &mut VecDeque<Command>) {
    while let Some(command) = commands.pop_front() {
        match command {
            Command::ChangeDir(dir) => {
                match pwd.directories.iter_mut().filter(|d| d.name.eq(&dir)).next() {
                    Some(mut existing) => {
                        consume_commands(&mut existing, commands);
                    },
                    None => {
                        let mut new = Directory { name: dir, directories: Vec::new(), files: Vec::new() };
                        consume_commands(&mut new, commands);
                        pwd.directories.push(new);
                    }
                }
            },
            Command::ChangeRoot => {
                if !pwd.name.eq("") {
                    commands.push_front(command);
                    break;
                }
            },
            Command::ChangeBack => {
                break;
            },
            Command::List(list) => {
                for line in list {
                    let words: Vec<&str> = line.split(" ").collect();
                    if words.len() != 2 {
                        panic!("Should contain 2 words");
                    }
                    if words[0].eq("dir") {
                        pwd.directories.push(Directory { name: words[1].to_owned(), files: Vec::new(), directories: Vec::new() })
                    } else {
                        pwd.files.push(File { name: words[1].to_owned(), size: words[0].parse().unwrap()});
                    }
                }
            }
        }
    }
}

fn find_smallest_dir(pwd: &Directory, min_size: usize) -> usize {
    let mut smallest = get_dir_size(pwd);
    if smallest >= min_size {
        for dir in &pwd.directories {
            let candidate = find_smallest_dir(dir, min_size);
            if candidate < smallest && candidate >= min_size {
                smallest = candidate;
            }
        }
    }
    smallest
}