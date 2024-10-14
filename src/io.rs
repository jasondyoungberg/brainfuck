use core::slice;
use std::{
    io::{stdin, stdout, Read, Result, Write},
    num::IntErrorKind,
    process::exit,
};

pub enum Reader {
    Standard,
    Number,
    Echo,
    Raw,
}

pub enum Writer {
    Standard,
    Number,
}

impl Read for Reader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self {
            Reader::Standard => stdin().read(buf),
            Reader::Number => number_read(buf),
            Reader::Echo => echo_read(buf),
            Reader::Raw => raw_read(buf),
        }
    }
}
impl Write for Writer {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match self {
            Writer::Standard => stdout().write(buf),
            Writer::Number => number_write(buf),
        }
    }

    fn flush(&mut self) -> Result<()> {
        match self {
            Writer::Standard => stdout().flush(),
            Writer::Number => number_flush(),
        }
    }
}

fn echo_read(buf: &mut [u8]) -> Result<usize> {
    let read = stdin().read(buf)?;

    let output = buf[0..read]
        .iter()
        .flat_map(|x| -> &[u8] {
            match x {
                3 => exit(0), // ctrl c
                13 => b"\r\n",
                _ => slice::from_ref(x),
            }
        })
        .cloned()
        .collect::<Box<_>>();

    stdout().write_all(&output)?;
    stdout().flush()?;
    Ok(read)
}

fn raw_read(buf: &mut [u8]) -> Result<usize> {
    let read = stdin().read(buf)?;
    if buf[0..read].contains(&3) {
        exit(0)
    } else {
        Ok(read)
    }
}

fn number_write(buf: &[u8]) -> Result<usize> {
    assert_eq!(buf.len(), 1, "only one byte can be written at a time");
    println!("output: {}", buf[0]);
    Ok(1)
}

fn number_flush() -> Result<()> {
    Ok(())
}

fn number_read(buf: &mut [u8]) -> Result<usize> {
    assert_eq!(buf.len(), 1, "only one byte can be read at a time");

    buf[0] = loop {
        print!("input: ");
        stdout().flush().unwrap_or_else(|err| {
            eprintln!("error while flushing stdout:\n{err}");
            exit(1);
        });

        let mut line = String::new();
        stdin().read_line(&mut line)?;

        match line.trim().parse() {
            Ok(x) => break x,
            Err(err) => match err.kind() {
                IntErrorKind::NegOverflow | IntErrorKind::PosOverflow => {
                    println!("input must be within 0 - 255")
                }
                IntErrorKind::Empty => println!("please provide an input"),
                IntErrorKind::InvalidDigit => println!("input must be a number"),
                IntErrorKind::Zero => unreachable!(),
                _ => println!("error while parsing input: {err}"),
            },
        }
    };

    Ok(1)
}
