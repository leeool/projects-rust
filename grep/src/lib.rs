use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string(config.input)?;

    println!("{:?}", config.insen);

    match config.insen {
        true => {
            let search = search_insensitive(&config.query, &file);
            for (i, occ) in search.occurrences.iter().enumerate() {
                println_colorized(occ, search.line_occurrences[i])
            }
        }
        false => {
            let search = search_sensitive(&config.query, &file);
            for (i, occ) in search.occurrences.iter().enumerate() {
                println_colorized(occ, search.line_occurrences[i]);
            }
        }
    }

    Ok(())
}

fn println_colorized(value: &str, line_value: usize) -> () {
    println!("{} \x1b[93m{}\x1b[0m", line_value, value);
}

struct Search<'a> {
    occurrences: Vec<&'a str>,
    line_occurrences: Vec<usize>,
}

fn search_sensitive<'a>(query: &str, content: &'a str) -> Search<'a> {
    let mut occurrences: Vec<&'a str> = Vec::new();
    let mut line_occurrences: Vec<usize> = Vec::new();

    for (pos, line) in content.lines().enumerate() {
        if line.contains(query) {
            occurrences.push(line);
            line_occurrences.push(pos + 1);
        }
    }

    Search {
        occurrences,
        line_occurrences,
    }
}

fn search_insensitive<'a>(query: &str, content: &'a str) -> Search<'a> {
    let mut line_occurrences: Vec<usize> = Vec::new();
    let mut occurrences = Vec::new();
    let query = query.to_lowercase();

    for (pos, line) in content.lines().enumerate() {
        if line.to_lowercase().contains(&query) {
            occurrences.push(line);
            line_occurrences.push(pos + 1);
        }
    }

    Search {
        occurrences,
        line_occurrences,
    }
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
            search_sensitive(query, content).occurrences
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
            search_insensitive(query, content).occurrences
        );
    }

    #[test]
    fn line_number() {
        let content = "\
Muitas vezes esta frase é atribuida a Sócrates:
'Conhece-te a ti mesmo'.
O melhor filósofo é aquele capaz de se encantar com as coisas.
        ";
        let query = "sócrates";

        assert_eq!(vec![1], search_insensitive(query, content).line_occurrences);
    }
}
