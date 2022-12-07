use std::env;
use std::fs;
use std::str::FromStr;

struct File {
    name: String,
    size: usize
}

struct Directory {
    name: String,
    files: Vec<File>,
    directories: Vec<Directory>
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
            let dir = &text[3..];
            if dir.eq("..") {
                Ok(Command::ChangeBack)
            } else if dir.eq("/") {
                Ok(Command::ChangeRoot)
            } else {
                Ok(Command::ChangeDir(dir.to_owned()))
            }
        } else if text.starts_with("ls ") {
            let lines = text.split("\r\n").skip(1).map(|s| s.to_owned()).collect();
            Ok(Command::List(lines))
        } else {
            Err(format!("Command not found: {}", text))
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let commands: Vec<Command> = text.split("$ ").map(|c| c.parse().unwrap()).collect();
        let root: Directory = parse_filesystem(commands);
        println!("Number of dirs in root: {}", root.directories.len());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn parse_filesystem(commands: Vec<Command>) -> Directory {
    panic!("TODO")
}
