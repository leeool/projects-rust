use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string(config.input)?;

    println!("{:?}", config.insen);

    if config.insen {
        for occurencie in search_insensitive(&config.query, &file) {
            println!("{}", occurencie);
        }
    } else {
        for occurencie in search_sensitive(&config.query, &file) {
            println!("{}", occurencie);
        }
    }

    Ok(())
}

fn search_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut occurencies = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            occurencies.push(line)
        }
    }

    occurencies
}

fn search_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut occurencies = Vec::new();
    let query = query.to_lowercase();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            occurencies.push(line)
        }
    }

    occurencies
}

#[derive(Debug, Default)]
pub struct Config {
    pub input: String,
    pub query: String,
    pub insen: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        println!("{:?}", args);

        let mut arguments = Config {
            insen: false,
            ..Config::default()
        };

        for i in 1..args.len() {
            match args[i].as_ref() {
                "i" => arguments.input = args[i + 1].clone(),
                "q" => arguments.query = args[i + 1].clone(),
                "insen" => arguments.insen = true,
                _ => (),
            }
        }

        Ok(Config {
            input: arguments.input,
            query: arguments.query,
            insen: arguments.insen,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let content = "\
Muitas vezes esta frase é atribuida a Sócrates:
'Conhece-te a ti mesmo'.
Ódio ao TI.
        ";
        let query = "ti";

        assert_eq!(
            vec!["'Conhece-te a ti mesmo'."],
            search_sensitive(query, content)
        );
    }

    #[test]
    fn case_insensitive() {
        let content = "\
Muitas vezes esta frase é atribuida a Sócrates:
'Conhece-te a ti mesmo'.
        ";
        let query = "sócrates";

        assert_eq!(
            vec!["Muitas vezes esta frase é atribuida a Sócrates:"],
            search_insensitive(query, content)
        );
    }
}
