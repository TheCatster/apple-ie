use assembler::assemble;
use cpu::Cpu;

use anyhow::Result;
use chrono::Local;
use clap::Parser;
use fern::{log_file, Dispatch};
use log::{info, LevelFilter};
use std::{fs, io};

pub mod assembler;
pub mod cpu;
pub mod memory;

#[cfg(test)]
mod tests;

/// Define a struct to hold command line arguments.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on.
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    /// Specify an assembly file to run.
    #[arg(short, long)]
    file: Option<String>,
}

/// The main function of the program.
///
/// # Returns
///
/// A `Result` containing the result of running the program.
///
/// # Examples
///
/// ```
/// let result = apple_ie::main();
/// assert!(result.is_ok());
/// ```
fn main() -> Result<()> {
    // Parse command line arguments using the Cli struct
    let cli = Cli::parse();

    // Create a new CPU instance
    let mut cpu: Cpu = Cpu::new();

    // Configure the logger using Fern
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Info)
        .chain(io::stdout())
        .chain(log_file("apple-ie.log").expect("No permission to write to the current directory."))
        .apply()
        .expect("Failed to dispatch Fern logger!");

    // Log that the logger has been initialised
    info!("Logging initialised.");

    // Read the program from a file or use a default program
    let program = match &cli.file {
        Some(file) => fs::read_to_string(file)?,
        None => String::from(
            "LDA #$01
             BRK",
        ),
    };

    // Assemble the program into machine code
    let bytes = assemble(&program)?;

    // Log that the program has been assembled
    info!("Program assembled.");

    // Load the program into memory
    cpu.load(0x800, &bytes);

    // Log that the program has been loaded into memory
    info!("Program loaded into memory.");

    // Run the program on the CPU
    cpu.run()?;

    Ok(())
}