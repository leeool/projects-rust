use std::env;
use std::process;
use grep::Config;
use grep::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("search in: {:?}", config.input);
    println!("query: {:?}", config.query);

    run(config).unwrap_or_else(|err| {
        println!("Application error: {}", err);
        process::exit(1);
    });
}

