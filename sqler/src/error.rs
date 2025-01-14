use crate::lexer::TokenKind;
use std::ops::Range;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unexpected token: expected {expected:?}, found {found:?}")]
    UnexpectedToken {
        expected: TokenKind,
        found: TokenKind,
        span: Range<usize>,
    },

    #[error("Unexpected end of input")]
    UnexpectedEOF,

    #[error("Invalid syntax: {0}")]
    InvalidSyntax(String),
}
