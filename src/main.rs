use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::error::Error;

#[derive(Debug)]
struct Config {
    servers: Vec<hyper::Uri>,
    weights: Vec<u32>,
    algo: Algorithm,
}

impl Config {
    fn new() -> Self {
        Config {
            servers: Vec::new(),
            weights: Vec::new(),
            algo: Algorithm::round_robin, // using round robin as default algorithm
        }
    }
    fn update(&mut self, path: &str) -> io::Result<&Config> {
        let path = Path::new(path);
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            if line.starts_with("servers:") {
                let servers = line.trim_start_matches("servers:").trim();
                self.servers = servers
                                    .split(",")
                                    .map(|server| server
                                        .trim()
                                        .parse::<hyper::Uri>()
                                        .expect("Invalid URI"))
                                    .collect();
            } else if line.starts_with("weights:") {
                let weights = line.trim_start_matches("weights:").trim();
                self.weights = weights
                                    .split(",")
                                    .map(|weight| weight
                                        .trim()
                                        .parse::<u32>()
                                        .expect("Invalid weight"))
                                    .collect();
            } else if line.starts_with("algorithm:") {
                let algo = line.trim_start_matches("algorithm:").trim();
                self.algo = match algo {
                    "round_robin" => Algorithm::round_robin,
                    "weighted_round_robin" => Algorithm::weighted_round_robin,
                    "least_connections" => Algorithm::least_connections,
                    "weighted_least_connections" => Algorithm::weighted_least_connections,
                    "least_response_time" => Algorithm::least_response_time,
                    "weighted_least_response_time" => Algorithm::weighted_least_response_time,
                    _ => Algorithm::round_robin, // Default algorithms
                }
            }
        }

        Ok(self)
    }
}

#[derive(Debug)]
enum Algorithm {
    round_robin,
    weighted_round_robin,
    least_connections,
    weighted_least_connections,
    least_response_time,
    weighted_least_response_time,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut config = Config::new();
    let ref_config = config.update("src/.config")?;

    println!("{:?}", ref_config); // remove

    // cli

    Ok(())
}