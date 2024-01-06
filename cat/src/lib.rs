use std::env::args;
use std::fs::read_to_string;
use std::path::Path;
use std::process;

pub fn run() {
    let args = Parser::new().unwrap_or_else(|err| {
        println!("Something went wrong: {}", err);
        process::exit(1);
    });

    if args.numbers {
        concat_numbers(args);
    } else if args.numbers_non_blank {
        concat_no_blank(args);
    } else {
        concat(args)
    }
}

fn concat(args: Args) -> () {
    let file_lines = read_file(args.file_path.as_str());

    for line in file_lines {
        print!("{}\n", line);
    }
}

fn concat_numbers(args: Args) {
    let file_lines = read_file(args.file_path.as_str());

    for (i, line) in file_lines.iter().enumerate() {
        print!("{} {}\n", i + 1, line);
    }
}

fn concat_no_blank(args: Args) {
    let file_lines = read_file(args.file_path.as_str());

    for (i, line) in file_lines.iter().enumerate() {
        if line.is_empty() {
            print!("\n");
            continue;
        }
        print!("{} {}\n", i + 1, line);
    }
}

fn read_file<'a>(file_path: &str) -> Vec<String> {
    let mut file_lines = Vec::new();
    let mut file_numbers = Vec::new();
    let file_as_string_result = read_to_string(file_path);

    let file_as_string = match file_as_string_result {
        Ok(file) => file,
        Err(e) => {
            println!("Unable to read file: {}", e);
            process::exit(1);
        }
    };

    for (i, line) in file_as_string.lines().enumerate() {
        file_lines.push(line.to_string());
        file_numbers.push(i + 1);
    }

    file_lines
}

#[derive(Debug, Default)]
pub struct Args {
    pub file_path: String,
    pub numbers: bool,
    pub numbers_non_blank: bool,
}

pub struct Parser {
    pub args: Vec<Args>,
}

impl Parser {
    pub fn new() -> Result<Args, &'static str> {
        let args = args().skip(1).collect::<Vec<String>>();
        let mut arguments = Args::default();

        println!("paths: {:?}", args);

        if args.len() < 1 {
            return Err("Not enough arguments");
        }

        for i in 0..args.len() {
            let is_file = Path::new(&args[i]).is_file();

            if is_file {
                arguments.file_path = args[i].clone();
            }

            if args[i] == "-b" {
                arguments.numbers_non_blank = true;
                arguments.numbers = false
            }

            if args[i] == "-n" {
                arguments.numbers = true;
            }
        }

        Ok(Args { ..arguments })
    }
}
