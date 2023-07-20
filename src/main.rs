use assembler::assemble;
use cpu::CPU;

use anyhow::Result;
use chrono::Local;
use clap::Parser;
use fern::{log_file, Dispatch};
use log::{info, LevelFilter};
use std::{fs, io};

mod assembler;
mod cpu;
mod memory;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    /// Specify an assembly file to run
    #[arg(short, long)]
    file: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut cpu: CPU = CPU::new();

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

    info!("Logging initialised.");

    let program = match &cli.file {
        Some(file) => fs::read_to_string(file)?,
        None => String::from(
            "LDA #$c0
             TAX
             INX
             ADC #$c4
             BRK",
        ),
    };

    let bytes = assemble(&program);

    info!("Program assembled.");

    cpu.load(0x800, &bytes);

    info!("Program loaded into memory.");

    cpu.run();

    Ok(())
}
