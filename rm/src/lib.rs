#[derive(Default, Debug)]
pub struct Config {
    pub directory: String,
    pub recursively: bool,
    pub force: bool,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, String> {
        let mut configs = Config::default();
        let mut flags: Vec<&str> = Vec::new();

        if args.is_empty() {
            return Err(String::from("Invalid args length"));
        };

        let full_args = args.join(" ");
        let chars = full_args.split("").skip(1).collect::<Vec<&str>>();

        let mut index = 0;
        while index < chars.len() {
            if chars[index] == "-" {
                for j in index..chars.len() {
                    if chars[j] == " " {
                        break;
                    }
                    index += 1;
                    flags.push(chars[j]);
                }
                continue;
            }

            if configs.directory.is_empty() {
                for j in index..chars.len() {
                    if chars[j] == " " {
                        break;
                    }
                    configs.directory += chars[j];
                }
            }

            index += 1;
        }

        for flag in flags.iter() {
            match *flag {
                "r" => configs.recursively = true,
                "f" => configs.force = true,
                _ => (),
            }
        }

        if configs.directory.is_empty() {
            return Err(String::from("Directory missing"))
        }

        Ok(configs)
    }
}
