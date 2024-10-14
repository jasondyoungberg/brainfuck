use crate::{
    error::{Location, ParseError, ParseErrorKind},
    token::{Basic, Token},
};

#[derive(Debug, Clone)]
pub enum Action {
    Basic(Basic),
    Loop(Box<[Action]>),
}

pub fn parse(
    tokens: &mut impl Iterator<Item = (Location, Token)>,
) -> Result<Box<[Action]>, ParseError> {
    let mut actions = Vec::new();

    while let Some((loc, token)) = tokens.next() {
        match token {
            Token::Basic(b) => actions.push(Action::Basic(b)),
            Token::StartLoop => actions.push(Action::Loop(parse_loop(tokens, loc)?)),
            Token::EndLoop => {
                return Err(ParseError {
                    loc,
                    kind: ParseErrorKind::UnmatchedLoopEnd,
                })
            }
        }
    }

    Ok(actions.into_boxed_slice())
}

fn parse_loop(
    tokens: &mut impl Iterator<Item = (Location, Token)>,
    start_loc: Location,
) -> Result<Box<[Action]>, ParseError> {
    let mut actions = Vec::new();

    while let Some((loc, token)) = tokens.next() {
        match token {
            Token::Basic(b) => actions.push(Action::Basic(b)),
            Token::StartLoop => actions.push(Action::Loop(parse_loop(tokens, loc)?)),
            Token::EndLoop => return Ok(actions.into_boxed_slice()),
        }
    }

    Err(ParseError {
        loc: start_loc,
        kind: ParseErrorKind::UnmatchedLoopStart,
    })
}
