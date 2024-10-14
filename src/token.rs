use crate::error::Location;

#[derive(Debug, Clone, Copy)]
pub enum Basic {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Output,
    Input,
}

#[derive(Debug, Clone, Copy)]
pub enum Token {
    Basic(Basic),
    StartLoop,
    EndLoop,
}

pub fn tokenize(code: &str) -> impl Iterator<Item = (Location, Token)> + '_ {
    code.lines()
        .enumerate()
        .flat_map(|(line_num, line)| {
            line.chars().enumerate().map(move |(col, char)| {
                (
                    Location {
                        line: line_num,
                        col,
                    },
                    char,
                )
            })
        })
        .filter_map(|(loc, c)| {
            Some((
                loc,
                match c {
                    '>' => Token::Basic(Basic::MoveRight),
                    '<' => Token::Basic(Basic::MoveLeft),
                    '+' => Token::Basic(Basic::Increment),
                    '-' => Token::Basic(Basic::Decrement),
                    '.' => Token::Basic(Basic::Output),
                    ',' => Token::Basic(Basic::Input),
                    '[' => Token::StartLoop,
                    ']' => Token::EndLoop,
                    _ => return None,
                },
            ))
        })
}
