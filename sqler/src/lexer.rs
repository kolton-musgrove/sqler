use logos::Logos;
use std::ops::Range;

// represents a single input token
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Range<usize>,
}

// all t-sql tokens
#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\n\f+]")] // skip whitespace
pub enum TokenKind {
    // keywords
    #[token("SELECT", ignore(ascii_case))]
    Select,
    #[token("FROM", ignore(ascii_case))]
    From,
    #[token("WHERE", ignore(ascii_case))]
    Where,
    #[token("JOIN", ignore(ascii_case))]
    Join,
    #[token("ON", ignore(ascii_case))]
    On,
    #[token("GROUP", ignore(ascii_case))]
    Group,
    #[token("BY", ignore(ascii_case))]
    By,
    #[token("ORDER", ignore(ascii_case))]
    Order,
    #[token("HAVING", ignore(ascii_case))]
    Having,
    #[token("AS", ignore(ascii_case))]
    As,

    // asterisk
    #[token("*")]
    Asterisk,

    // operators
    #[token("=")]
    Equals,
    #[token("<")]
    LessThan,
    #[token(">")]
    GreaterThan,
    #[token("<=")]
    LessEquals,
    #[token("=>")]
    GreaterEquals,
    #[token("<>")]
    NotEquals,

    // punctuation
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("(")]
    LParens,
    #[token(")")]
    RParens,

    // literals
    #[regex("[0-9]+")]
    Number,
    #[regex(r#"'[^']*'"#)]
    String,

    // identifiers - including quoted identifiers
    #[regex(r#"[a-zA-Z_][a-zA-Z0-9_]*|\[[^\]]*\]"#)]
    Identifier,
}

// main lexer type for tokenizing sql input
pub struct Lexer<'a> {
    logos_lexer: logos::Lexer<'a, TokenKind>,
}

impl<'a> Lexer<'a> {
    // create a new lexer from input text
    pub fn new(input: &'a str) -> Self {
        Self {
            logos_lexer: TokenKind::lexer(input),
        }
    }

    // get the next token from input
    pub fn next_token(&mut self) -> Option<Token> {
        let kind = Result::expect(self.logos_lexer.next()?, "failed to identify token.");
        let span = self.logos_lexer.span();

        Some(Token { kind, span })
    }

    // peek at the next token without consuming it
    pub fn peek_token(&mut self) -> Option<Token> {
        let _ = self.logos_lexer.span();
        let current_state = self.logos_lexer.clone();

        let token = self.next_token();
        self.logos_lexer = current_state;

        token
    }
}
