mod generators;

use clap::{Arg, Command};
use generators::XorShift64;
use std::fs::File;
use std::io::{BufWriter, Write};

const DEFAULT_COUNT: u128 = 10_000_000;
const DEFAULT_SEED: u64 = 10442988001049997600;

fn main() -> std::io::Result<()> {
    let matches = Command::new("random diehard-proof")
        .version("0.1.0")
        .about("Teaches args")
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Defines output file"),
        )
        .arg(
            Arg::new("seed")
                .short('s')
                .long("seed")
                .help("Defines seed for a certain generator"),
        )
        .arg(
            Arg::new("count")
                .short('n')
                .long("amount")
                .help("Defines how many loops generator should go through"),
        )
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .help("Lists all available generators")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    // Handle list flag early
    if matches.get_flag("list") {
        let other_args = ["output", "seed", "count"];
        let has_other_args = other_args
            .iter()
            .any(|arg| matches.contains_id(arg) && matches.get_one::<String>(arg).is_some());

        if has_other_args {
            eprintln!(
                "Error: --list cannot be used with other arguments like --output, --seed, or --amount."
            );
            std::process::exit(1);
        }

        println!("Available generators:");
        println!(" - XorShift64");
        return Ok(());
    }

    // Normal generator workflow
    let output_path = matches
        .get_one("output")
        .map(String::as_str)
        .unwrap_or("output.bin");

    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);

    let seed = matches
        .get_one::<String>("seed")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(DEFAULT_SEED);

    println!("Using seed: {}", seed);
    let count = matches
        .get_one::<String>("count")
        .and_then(|s| s.parse::<u128>().ok())
        .unwrap_or(DEFAULT_COUNT);

    let r#gen = XorShift64::new(seed, count);

    for x in r#gen {
        writer.write_all(&x.to_ne_bytes())?;
    }

    println!("Successfully generated random numbers");

    Ok(())
}
