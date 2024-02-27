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

        for i in 0..chars.len() {
            if chars[i] != "-" {
                continue;
            }

            for j in i..chars.len() {
                if chars[j] == " " {
                    break;
                }
                flags.push(chars[j]);
            }
        }

        for flag in flags.iter() {
            match *flag {
                "r" => configs.recursively = true,
                "f" => configs.force = true,
                _ => (),
            }
        }

        println!("{:?}", configs);

        todo!("should parse args return and config")
    }
}
