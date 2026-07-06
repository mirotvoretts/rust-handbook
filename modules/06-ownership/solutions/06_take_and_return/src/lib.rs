//! 06 (1x) — Consuming builder. Эталонное решение.

#[derive(Debug, PartialEq)]
pub struct Config {
    pub name: String,
    pub verbose: bool,
    pub retries: u32,
}

impl Config {
    pub fn new() -> Config {
        Config {
            name: String::new(),
            verbose: false,
            retries: 0,
        }
    }

    pub fn with_name(mut self, name: &str) -> Config {
        self.name = name.to_string();
        self
    }

    pub fn with_verbose(mut self, verbose: bool) -> Config {
        self.verbose = verbose;
        self
    }

    pub fn with_retries(mut self, retries: u32) -> Config {
        self.retries = retries;
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new()
    }
}
