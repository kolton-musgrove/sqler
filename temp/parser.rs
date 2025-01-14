// sql-formatter/src/parser.rs
use crate::ast::*;
use crate::error::ParseError;
use crate::lexer::{Lexer, Token, TokenKind};
use std::ops::Range;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Option<Token>,
    peek: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let current = lexer.next_token();
        let peek = lexer.next_token();

        Self {
            lexer,
            current,
            peek,
        }
    }

    fn advance(&mut self) -> Option<Token> {
        let next = self.lexer.next_token();
        let current = self.peek.take();
        self.peek = next;
        std::mem::replace(&mut self.current, current)
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Token, ParseError> {
        match self.current {
            Some(ref token) if token.kind == kind => Ok(self.advance().unwrap()),
            Some(ref token) => Err(ParseError::UnexpectedToken {
                expected: kind,
                found: token.kind.clone(),
                span: token.span.clone(),
            }),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    pub fn parse_select(&mut self) -> Result<SelectStatement, ParseError> {
        let start_span = self.expect(TokenKind::Select)?.span;

        // Parse columns
        let mut columns = Vec::new();
        loop {
            columns.push(self.parse_select_column()?);

            if !matches!(
                self.current,
                Some(Token {
                    kind: TokenKind::Comma,
                    ..
                })
            ) {
                break;
            }
            self.advance(); // Consume comma
        }

        // Parse FROM clause
        self.expect(TokenKind::From)?;
        let from = self.parse_table_reference()?;

        // Parse optional WHERE clause
        let where_clause = if matches!(
            self.current,
            Some(Token {
                kind: TokenKind::Where,
                ..
            })
        ) {
            self.advance(); // Consume WHERE
            Some(self.parse_where_clause()?)
        } else {
            None
        };

        // Parse optional GROUP BY
        let group_by = if matches!(
            self.current,
            Some(Token {
                kind: TokenKind::Group,
                ..
            })
        ) {
            self.advance(); // Consume GROUP
            self.expect(TokenKind::By)?;
            Some(self.parse_expression_list()?)
        } else {
            None
        };

        let end_span = match self.current {
            Some(ref token) => token.span.clone(),
            None => start_span.clone(),
        };

        Ok(SelectStatement {
            span: Span::from(start_span.start..end_span.end),
            columns,
            from,
            where_clause,
            group_by,
            having: None,   // TODO: Implement HAVING clause parsing
            order_by: None, // TODO: Implement ORDER BY clause parsing
        })
    }

    fn parse_select_column(&mut self) -> Result<SelectColumn, ParseError> {
        let expr = self.parse_expression()?;

        // Check for optional AS alias
        let alias = if matches!(
            self.current,
            Some(Token {
                kind: TokenKind::Identifier,
                ..
            })
        ) {
            let token = self.advance().unwrap();
            Some(self.get_identifier_text(&token))
        } else {
            None
        };

        Ok(SelectColumn {
            span: expr.span(),
            expr,
            alias,
        })
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        // Start with parsing the highest precedence expressions
        self.parse_primary_expression()
    }

    fn parse_table_reference(&mut self) -> Result<TableReference, ParseError> {
        let start_token = self.current.clone().ok_or(ParseError::UnexpectedEOF)?;
        let table_name = self.expect(TokenKind::Identifier)?;

        // Check for optional alias
        let alias = if matches!(
            self.current,
            Some(Token {
                kind: TokenKind::Identifier,
                ..
            })
        ) {
            Some(self.advance().unwrap())
        } else {
            None
        };

        Ok(TableReference {
            span: Span::from(
                start_token.span.start
                    ..alias
                        .as_ref()
                        .map(|t| t.span.end)
                        .unwrap_or(table_name.span.end),
            ),
            name: self.get_identifier_text(&table_name),
            alias: alias.map(|t| self.get_identifier_text(&t)),
            schema: None, // TODO: Handle schema qualification
        })
    }

    fn parse_where_clause(&mut self) -> Result<WhereClause, ParseError> {
        let start_span = match self.current {
            Some(ref token) => token.span.clone(),
            None => return Err(ParseError::UnexpectedEOF),
        };

        let condition = self.parse_expression()?;
        let end_span = condition.span();

        Ok(WhereClause {
            span: Span::from(start_span.start..end_span.end),
            condition,
        })
    }

    fn parse_expression_list(&mut self) -> Result<Vec<Expression>, ParseError> {
        let mut expressions = Vec::new();

        loop {
            expressions.push(self.parse_expression()?);

            if !matches!(
                self.current,
                Some(Token {
                    kind: TokenKind::Comma,
                    ..
                })
            ) {
                break;
            }
            self.advance(); // Consume comma
        }

        Ok(expressions)
    }

    fn parse_primary_expression(&mut self) -> Result<Expression, ParseError> {
        match self.current.take() {
            Some(Token {
                kind: TokenKind::Identifier,
                span,
            }) => {
                self.advance();
                Ok(Expression::Column {
                    span: span.into(),
                    name: self.get_identifier_text(&Token {
                        kind: TokenKind::Identifier,
                        span: span.clone(),
                    }),
                    table: None,
                })
            }
            Some(Token {
                kind: TokenKind::String,
                span,
            }) => {
                self.advance();
                Ok(Expression::Literal {
                    span: span.into(),
                    value: LiteralValue::String(self.get_string_literal(&span)),
                })
            }
            Some(Token {
                kind: TokenKind::Number,
                span,
            }) => {
                self.advance();
                Ok(Expression::Literal {
                    span: span.into(),
                    value: LiteralValue::Number(self.get_number_literal(&span)),
                })
            }
            Some(token) => Err(ParseError::UnexpectedToken {
                expected: TokenKind::Identifier,
                found: token.kind,
                span: token.span,
            }),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    // Helper methods
    fn get_identifier_text(&self, token: &Token) -> String {
        // TODO: Implement proper identifier text extraction
        "identifier".to_string()
    }

    fn get_string_literal(&self, span: &Range<usize>) -> String {
        // TODO: Implement proper string literal extraction
        "string".to_string()
    }

    fn get_number_literal(&self, span: &Range<usize>) -> String {
        // TODO: Implement proper number literal extraction
        "0".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_select() {
        let input = "SELECT id, name FROM users";
        let mut parser = Parser::new(input);

        let result = parser.parse_select();
        assert!(result.is_ok());

        let select = result.unwrap();
        assert_eq!(select.columns.len(), 2);
    }
}
