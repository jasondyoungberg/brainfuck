//! # Brainfuck Programming Language
//! `>` Increment the data pointer by one (to point to the next cell to the right).
//! `<` Decrement the data pointer by one (to point to the next cell to the left).
//! `+` Increment the byte at the data pointer by one.
//! `-` Decrement the byte at the data pointer by one.
//! `.` Output the byte at the data pointer.
//! `,` Accept one byte of input, storing its value in the byte at the data pointer.
//! `[` If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command.
//! `]` If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.[a]

pub mod error;
pub mod io;
pub mod parser;
pub mod runner;
pub mod tape;
pub mod token;

use std::{env, fs, process::exit};

use crossterm::{
    style::Stylize,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use io::{Reader, Writer};
use parser::parse;
use runner::execute;
use token::tokenize;

fn main() {
    disable_raw_mode().unwrap_or_else(|err| {
        eprintln!("{}", format!("error while disabling raw mode: {err}").red())
    });

    let args = env::args().collect::<Vec<_>>();

    if args.len() != 2 {
        eprintln!("{}", format!("usage: {} [file]", args[0]).cyan());
        exit(2);
    }

    let filename = &args[1];

    let code_bytes = fs::read(filename).unwrap_or_else(|err| {
        eprintln!("{}", format!("error reading file {filename}:\n{err}").red());
        exit(1);
    });

    let code = String::from_utf8(code_bytes).unwrap_or_else(|err| {
        eprintln!("invalid unicode in {filename}:\n{err}");
        exit(1);
    });

    let parsed = parse(&mut tokenize(&code)).unwrap_or_else(|err| {
        let kind = err.kind;
        let loc = err.loc;
        eprintln!(
            "{}",
            format!("error while parsing code:\n{kind} at {filename}:{loc}").red()
        );
        exit(1);
    });

    let mut input = Reader::Standard;
    let mut output = Writer::Standard;

    if matches!(input, Reader::Echo | Reader::Raw) {
        enable_raw_mode().unwrap_or_else(|err| {
            eprintln!("{}", format!("error while enabling raw mode: {err}").red())
        });
    }

    execute(&parsed, &mut input, &mut output).unwrap_or_else(|err| {
        eprintln!("{}", format!("error while executing code:\n{err}").yellow());
        exit(1);
    });
}
