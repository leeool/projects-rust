use std::env::args;
use rm::Config;

fn main() -> Result<(), ()> {
    let args = args().skip(1).collect::<Vec<String>>();
    let config = match Config::new(args) {
        Ok(config) => config,
        Err(e) => return Err(println!("{}", e)) 
    };

    Ok(())
}
