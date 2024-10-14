use std::io::{self, Read, Write};

use crate::{parser::Action, tape::Tape, token::Basic};

pub fn execute(code: &[Action], input: &mut impl Read, output: &mut impl Write) -> io::Result<()> {
    let mut tape = Tape::new();

    for action in code.iter() {
        execute_impl(&mut tape, action.clone(), input, output)?;
    }
    Ok(())
}

fn execute_impl(
    tape: &mut Tape,
    code: Action,
    input: &mut impl Read,
    output: &mut impl Write,
) -> io::Result<()> {
    match code {
        Action::Basic(Basic::MoveRight) => tape.move_right(),
        Action::Basic(Basic::MoveLeft) => tape.move_left(),
        Action::Basic(Basic::Increment) => tape.increment(),
        Action::Basic(Basic::Decrement) => tape.decrement(),
        Action::Basic(Basic::Output) => {
            output.write_all(&[tape.read()])?;
            output.flush()?;
        }
        Action::Basic(Basic::Input) => {
            let mut buf = [0];
            input.read_exact(&mut buf)?;
            tape.write(buf[0]);
        }
        Action::Loop(actions) => {
            while tape.read() != 0 {
                for action in actions.iter() {
                    execute_impl(tape, action.clone(), input, output)?;
                }
            }
        }
    }
    Ok(())
}
