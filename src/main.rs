use clap::{Parser, Subcommand};
use std::{error::Error, fs, path::PathBuf};

pub(crate) mod diagnostics;
pub(crate) mod env;
pub(crate) mod interpreter;
pub(crate) mod lexer;
pub(crate) mod parser;
pub(crate) mod source;
pub(crate) mod vm;

pub(crate) type Result<T> = std::result::Result<T, Box<dyn Error>>;

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

fn main() -> crate::Result<()> {
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
