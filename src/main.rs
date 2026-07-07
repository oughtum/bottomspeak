use std::{error::Error, fs, path::PathBuf};

use bottomspeak_interp::interpreter;
use clap::{Parser, Subcommand};

#[derive(clap::Parser)]
#[command(about, disable_help_flag = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Evaluate and run a BottomSpeak source file.
    Run {
        #[arg()]
        file: PathBuf,
    },
    /// Launch an interactive REPL
    Repl,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    match args.command {
        Command::Run { file } => {
            let source = fs::read_to_string(&file)?;
            let name = file.to_str().unwrap_or("<source>");
            interpreter::run(&source, name)?
        }
        Command::Repl => interpreter::repl()?,
    }

    Ok(())
}
