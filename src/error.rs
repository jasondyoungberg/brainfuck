use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct ParseError {
    pub loc: Location,
    pub kind: ParseErrorKind,
}

#[derive(Debug, Clone)]
pub struct Location {
    pub line: usize,
    pub col: usize,
}
impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line + 1, self.col + 1)
    }
}

#[derive(Debug, Clone)]
pub enum ParseErrorKind {
    UnmatchedLoopStart,
    UnmatchedLoopEnd,
}
impl Display for ParseErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErrorKind::UnmatchedLoopStart => write!(f, "unmatched ["),
            ParseErrorKind::UnmatchedLoopEnd => write!(f, "unmatched ]"),
        }
    }
}
