use rm::{run, Config};
use std::{env::args, process};

fn main() -> Result<(), String> {
    let args = args().skip(1).collect::<Vec<String>>();
    let config = match Config::new(args) {
        Ok(config) => config,
        Err(e) => return Err(e),
    };

    match run(config) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
