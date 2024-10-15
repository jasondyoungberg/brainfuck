//! # Brainfuck Programming Language
//! `>` Increment the data pointer by one (to point to the next cell to the right).
//! `<` Decrement the data pointer by one (to point to the next cell to the left).
//! `+` Increment the byte at the data pointer by one.
//! `-` Decrement the byte at the data pointer by one.
//! `.` Output the byte at the data pointer.
//! `,` Accept one byte of input, storing its value in the byte at the data pointer.
//! `[` If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command.
//! `]` If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.[a]

mod assembler;
mod compiler;
mod error;
mod io;
mod parser;
mod runner;
mod tape;
mod token;

use std::{
    fmt::Display,
    fs,
    path::{Path, PathBuf},
    process::exit,
};

use clap::{Args, Parser, Subcommand, ValueEnum};

use assembler::assemble;
use compiler::compile;
use parser::parse;
use runner::execute;
use token::tokenize;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Arguments {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Compile the program
    Compile(CompileArgs),
    /// Interpret the program
    Interpret(InterpretArgs),
}

#[derive(Debug, Args)]
struct CompileArgs {
    /// The file to compile
    #[arg()]
    file: PathBuf,
    /// The file to write the compiled program to
    #[arg(long, short)]
    output: Option<PathBuf>,
    /// How to handle input and output
    #[arg(long, default_value_t = IoKind::Std)]
    io: IoKind,
}

#[derive(Debug, Args)]
struct InterpretArgs {
    /// The file to interpret
    #[arg()]
    file: PathBuf,
    /// How to handle input and output
    #[arg(long, default_value_t = IoKind::Std)]
    io: IoKind,
}

#[derive(Debug, Clone, ValueEnum)]
enum IoKind {
    /// Read and write directly to/from stdio
    Std,
    /// Read and write numbers
    Num,
}

impl Display for IoKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IoKind::Std => write!(f, "std"),
            IoKind::Num => write!(f, "num"),
        }
    }
}

fn main() {
    let args = Arguments::parse();

    let filename = match args.command {
        Commands::Compile(ref compile_args) => &compile_args.file,
        Commands::Interpret(ref interpret_args) => &interpret_args.file,
    }
    .as_path();

    let code_bytes = fs::read(filename).unwrap_or_else(|err| {
        eprintln!("error reading file {}:\n{}", filename.display(), err);
        exit(1);
    });

    let code = String::from_utf8(code_bytes).unwrap_or_else(|err| {
        eprintln!("invalid unicode in {}:\n{}", filename.display(), err);
        exit(1);
    });

    let parsed = parse(&mut tokenize(&code)).unwrap_or_else(|err| {
        eprintln!(
            "error while parsing code:\n{} at {}:{}",
            err.kind,
            filename.display(),
            err.loc
        );
        exit(1);
    });

    match args.command {
        Commands::Compile(compile_args) => {
            let asm = compile(&parsed, &compile_args.io);
            assemble(
                &asm,
                compile_args.output.as_deref().unwrap_or(Path::new("a.out")),
            )
            .unwrap_or_else(|err| {
                eprintln!("error while compiling:\n{err}");
                exit(1);
            });
        }
        Commands::Interpret(interpret_args) => {
            execute(&parsed, &mut interpret_args.io.clone()).unwrap_or_else(|err| {
                eprintln!("error while executing code:\n{err}");
                exit(1);
            });
        }
    }
}
