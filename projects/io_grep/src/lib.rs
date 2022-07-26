use std::{
  env,
  fs,
  error::Error
};

#[cfg(test)]
mod tests;

pub struct Config {
  pub query: String,
  pub filename: String,
  pub ignore_case: bool,
}

impl Config {
  pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
      args.next();

      let query = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a query string"),
      };

      let filename = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a file name"),
      };

      let ignore_case = env::var("IGNORE_CASE").is_ok();
  
      Ok(Config {
          query,
          filename,
          ignore_case
      })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;
  
  let results = if config.ignore_case {
    search_case_insensitive(&config.query, &contents)
  } else {
    search(&config.query, &contents)
  };

  for line in results {
    println!("{}", line);
  }

  Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();
  contents
    .lines()
    .filter(|line| line.to_lowercase().contains(&query))
    .collect()
}