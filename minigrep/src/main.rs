use std::{env, fs, string};

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let config = Config::new(&args);

    println!("Searching for {}!", config.query);
    println!("In file {}!", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read from file");

    println!("With text:\n, {} ", contents);
}
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        if args.len() > 3 {
            panic!("too many arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        return Config { query, file_path };
    }
}

// fn parse_config(args: &[String]) -> Config {
//     let query = &args[1].clone();
//     let file_path = &args[2].clone();
//     return Config {
//         query: query.to_string(),
//         file_path: file_path.to_string(),
//     };
// }
