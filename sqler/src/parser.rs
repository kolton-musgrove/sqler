use crate::ast::*;
use crate::error::ParseError;
use crate::lexer::{Lexer, Token, TokenKind};
use std::ops::Range;

pub struct Parser<'a> {
    input: &'a str,
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
            input,
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

        // parse columns
        let mut columns = Vec::new();
        loop {
            columns.push(self.parse_select_item()?);

            if !matches!(
                self.current,
                Some(Token {
                    kind: TokenKind::Comma,
                    ..
                })
            ) {
                break;
            }
            self.advance();
        }

        // parse FROM clause
        self.expect(TokenKind::From)?;
        let from = self.parse_table_reference()?;

        // parse optional WHERE clause
        let where_clause = if matches!(
            self.current,
            Some(Token {
                kind: TokenKind::Where,
                ..
            })
        ) {
            self.advance();
            Some(self.parse_where_clause()?)
        } else {
            None
        };

        // parse optional GROUP BY clause
        let group_by = if matches!(
            self.current,
            Some(Token {
                kind: TokenKind::Group,
                ..
            })
        ) {
            self.advance();
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
            having: None, // TODO: implement having clause parsing
                          // order_by: None, // TODO: implement ORDER BY clause parsing
        })
    }

    fn parse_select_item(&mut self) -> Result<SelectItem, ParseError> {
        match &self.current {
            Some(Token {
                kind: TokenKind::Asterisk,
                span,
            }) => {
                let span = span.clone();
                self.advance(); // consume asterisk
                Ok(SelectItem::Wildcard {
                    span: Span::from(span),
                })
            }
            Some(Token {
                kind: TokenKind::Identifier,
                ..
            }) => {
                let identifier = self.advance().unwrap();

                // check if it's a qualified wildcard (e.g. table.*)
                if matches!(
                    self.current,
                    Some(Token {
                        kind: TokenKind::Dot,
                        ..
                    })
                ) {
                    self.advance(); // consume dot

                    match &self.current {
                        Some(Token {
                            kind: TokenKind::Asterisk,
                            span,
                        }) => {
                            let end_span = span.clone();
                            self.advance(); // consume asterisk
                            Ok(SelectItem::QualifiedWildcard {
                                span: Span::from(identifier.span.start..end_span.end),
                                qualifier: self.get_identifier_text(&identifier),
                            })
                        }
                        _ => {
                            // regular qualified column
                            self.parse_regular_select_item(Some(identifier))
                        }
                    }
                } else {
                    // regular unqualified column
                    self.parse_regular_select_item(Some(identifier))
                }
            }
            Some(token) => Err(ParseError::UnexpectedToken {
                expected: TokenKind::Identifier,
                found: token.kind.clone(),
                span: token.span.clone(),
            }),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    fn parse_regular_select_item(
        &mut self,
        initial_identifier: Option<Token>,
    ) -> Result<SelectItem, ParseError> {
        let expr = if let Some(ident) = initial_identifier {
            Expression::Column {
                span: ident.span.clone().into(),
                name: self.get_identifier_text(&ident),
                table: None,
            }
        } else {
            self.parse_expression()?
        };

        // handle optional AS keyword and alias
        let alias = if matches!(
            self.current,
            Some(Token {
                kind: TokenKind::As,
                ..
            })
        ) {
            self.advance(); // consume AS
            let token = self.expect(TokenKind::Identifier)?;
            Some(self.get_identifier_text(&token))
        } else if matches!(
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

        Ok(SelectItem::Expression {
            span: expr.span(),
            expr,
            alias,
        })
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        // Parse the left side of any potential binary operation
        let mut left = self.parse_primary_expression()?;

        // Look for binary operators
        while let Some(token) = &self.current {
            let op = match token.kind {
                TokenKind::Equals => Some(Operator::Equals),
                TokenKind::NotEquals => Some(Operator::NotEquals),
                TokenKind::LessThan => Some(Operator::LessThan),
                TokenKind::GreaterThan => Some(Operator::GreaterThan),
                TokenKind::LessEquals => Some(Operator::LessEquals),
                TokenKind::GreaterEquals => Some(Operator::GreaterEquals),
                _ => None,
            };

            if let Some(operator) = op {
                let start_span = left.span();
                self.advance(); // consume the operator

                let right = self.parse_primary_expression()?;
                let end_span = right.span();

                left = Expression::BinaryOperation {
                    span: Span::from(start_span.start..end_span.end),
                    left: Box::new(left),
                    op: operator,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_table_reference(&mut self) -> Result<TableReference, ParseError> {
        let start_token = self.current.clone().ok_or(ParseError::UnexpectedEOF)?;

        // parse optional schema name
        let (schema, table_name) = if let Some(Token {
            kind: TokenKind::Identifier,
            ..
        }) = self.current
        {
            let schema_token = self.advance().unwrap();

            // check for schema separator
            if matches!(
                self.current,
                Some(Token {
                    kind: TokenKind::Dot,
                    ..
                })
            ) {
                self.advance(); // consume dot

                // parse table name
                let table_token = self.expect(TokenKind::Identifier)?;
                (Some(self.get_identifier_text(&schema_token)), table_token)
            } else {
                (None, schema_token)
            }
        } else {
            return Err(ParseError::UnexpectedToken {
                expected: TokenKind::Identifier,
                found: self
                    .current
                    .as_ref()
                    .map_or(TokenKind::Identifier, |t| t.kind.clone()),
                span: self.current.as_ref().map_or(0..0, |t| t.span.clone()),
            });
        };

        // check for optional alias
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
            schema,
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
            self.advance();
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
                    span: span.clone().into(),
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
                    span: span.clone().into(),
                    value: LiteralValue::String(self.get_string_literal(&span)),
                })
            }

            Some(Token {
                kind: TokenKind::Number,
                span,
            }) => {
                self.advance();
                Ok(Expression::Literal {
                    span: span.clone().into(),
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

    fn get_identifier_text(&self, token: &Token) -> String {
        let text = &self.input[token.span.clone()];
        // handle bracketed identifiers (e.g. [column1] -> column1)
        if text.starts_with('[') && text.ends_with(']') {
            text[1..text.len() - 1].to_string()
        } else {
            text.to_string()
        }
    }

    fn get_string_literal(&self, span: &Range<usize>) -> String {
        let text = &self.input[span.clone()];
        // remove surrounding quotes and trim whitespace
        if text.starts_with('\'') && text.ends_with('\'') {
            // TODO: handle escape sequences properly
            text[1..text.len() - 1].to_string()
        } else {
            text.to_string()
        }
    }

    fn get_number_literal(&self, span: &Range<usize>) -> String {
        self.input[span.clone()].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_table() {
        let input = "SELECT * FROM users";
        let mut parser = Parser::new(input);
        let result = parser.parse_select().unwrap();

        assert_eq!(result.from.name, "users");
        assert_eq!(result.from.schema, None);
        assert_eq!(result.from.alias, None);
    }

    #[test]
    fn test_schema_qualified_table() {
        let input = "SELECT * FROM dbo.users";
        let mut parser = Parser::new(input);
        let result = parser.parse_select().unwrap();

        assert_eq!(result.from.name, "users");
        assert_eq!(result.from.schema, Some("dbo".to_string()));
        assert_eq!(result.from.alias, None);
    }

    #[test]
    fn test_quoted_schema_and_table() {
        let input = "SELECT * FROM [HR Schema].[Employee Table] emp";
        let mut parser = Parser::new(input);
        let result = parser.parse_select().unwrap();

        assert_eq!(result.from.name, "Employee Table");
        assert_eq!(result.from.schema, Some("HR Schema".to_string()));
        assert_eq!(result.from.alias, Some("emp".to_string()));
    }

    #[test]
    fn test_invalid_schema_syntax() {
        let input = "SELECT * FROM dbo..Users";
        let mut parser = Parser::new(input);
        assert!(parser.parse_select().is_err());
    }

    #[test]
    fn test_identifier_extraction() {
        let input = "SELECT [My Table] FROM dbo.users";
        let parser = Parser::new(input);
        let token = Token {
            kind: TokenKind::Identifier,
            span: 7..17,
        };
        assert_eq!(parser.get_identifier_text(&token), "My Table");
    }

    #[test]
    fn test_basic_select() {
        let input = "SELECT column1 FROM table1";
        let mut parser = Parser::new(input);
        let result = parser.parse_select().unwrap();

        assert_eq!(result.columns.len(), 1);
        assert_eq!(result.from.name, "table1");
        assert!(result.where_clause.is_none());
        assert!(result.group_by.is_none());
    }

    // #[test]
    // fn test_select_with_alias() {
    //     let input = "SELECT column1 AS alias1 FROM table1";
    //     let mut parser = Parser::new(input);
    //     let result = parser.parse_select().unwrap();

    //     assert_eq!(result.columns.len(), 1);
    //     assert_eq!(result.columns[0].alias, Some("alias1".to_string()));
    // }

    #[test]
    fn test_select_multiple_columns() {
        let input = "SELECT column1, column2 FROM table1";
        let mut parser = Parser::new(input);
        let result = parser.parse_select().unwrap();

        assert_eq!(result.columns.len(), 2);
    }

    #[test]
    fn test_select_with_where_clause() {
        let input = "SELECT column1 FROM table1 WHERE column1 = 1";
        let mut parser = Parser::new(input);
        let result = parser.parse_select().unwrap();

        assert_eq!(result.columns.len(), 1);
        assert!(result.where_clause.is_some());
    }

    #[test]
    fn test_select_with_group_by() {
        let input = "SELECT column1 FROM table1 GROUP BY column1";
        let mut parser = Parser::new(input);
        let result = parser.parse_select().unwrap();

        assert_eq!(result.columns.len(), 1);
        assert!(result.group_by.is_some());
    }

    #[test]
    fn test_parse_error() {
        let input = "SELECT FROM";
        let mut parser = Parser::new(input);
        let result = parser.parse_select();

        assert!(result.is_err());
    }
}
