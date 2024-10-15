use std::{
    io::{stdin, stdout, Read, Result, Write},
    num::IntErrorKind,
    process::exit,
};

use crate::IoKind;

impl Read for IoKind {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self {
            IoKind::Std => std_read(buf),
            IoKind::Num => number_read(buf),
        }
    }
}
impl Write for IoKind {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match self {
            IoKind::Std => stdout().write(buf),
            IoKind::Num => number_write(buf),
        }
    }

    fn flush(&mut self) -> Result<()> {
        match self {
            IoKind::Std => stdout().flush(),
            IoKind::Num => Ok(()),
        }
    }
}

fn std_read(buf: &mut [u8]) -> Result<usize> {
    let mut buf = buf;

    loop {
        stdin().read_exact(buf)?;

        if !buf.contains(&b'\r') {
            break Ok(buf.len());
        }

        let filtered = buf
            .iter()
            .cloned()
            .filter(|c| *c != b'\r')
            .collect::<Vec<_>>();

        let (buf1, buf2) = buf.split_at_mut(filtered.len());

        buf1.copy_from_slice(&filtered);

        buf = buf2;
    }
}

fn number_write(buf: &[u8]) -> Result<usize> {
    assert_eq!(buf.len(), 1, "only one byte can be written at a time");
    println!("output: {}", buf[0]);
    Ok(1)
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
