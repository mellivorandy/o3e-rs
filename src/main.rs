mod simulator;
mod utils;

use std::env;

use crate::simulator::tomasulo::Tomasulo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = "output.txt";

    let mut simulator = match Tomasulo::from_file(input_file) {
        Ok(sim) => sim,
        Err(e) => {
            eprintln!("Failed to load file '{}': {}", input_file, e);
            std::process::exit(1);
        }
    };

    simulator.run(output_file);

    Ok(())
}
