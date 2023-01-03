use std::error::Error;
use std::fs;

pub struct Config<'a> {
    query: &'a String,
    file_paths: &'a [String],
}

impl<'a> Config<'a> {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() > 2 {
            let config = Config {
                query: &args[1],
                file_paths: &args[2..],
            };
            return Ok(config);
        } else {
            return Err("Expected a regex pattern followed by one or more file names");
        };
    }
}

pub fn run(config: Config) -> Result<String, Box<dyn Error>> {
    println!(
        "Searching for {} in {}...",
        config.query,
        config.file_paths.join(", ")
    );

    let contents = fs::read_to_string(&config.file_paths[0])
        .map_err(|err| format!("Error reading file \"{}\": {}", config.file_paths[0], err))?;
    return Ok(format!("With text:\n{}", contents));
}