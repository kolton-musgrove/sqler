fn parse_select_item(&mut self) -> Result<SelectItem, ParseError> {
    match &self.current {
        Some(Token {
            kind: TokenKind::Asterisk,
            span,
        }) => {
            let span = span.clone();
            self.advance(); // Consume *
            Ok(SelectItem::Wildcard { span })
        }
        Some(Token {
            kind: TokenKind::Identifier,
            ..
        }) => {
            let identifier = self.advance().unwrap();

            // Check if it's a qualified asterisk (table.*)
            if matches!(
                self.current,
                Some(Token {
                    kind: TokenKind::Dot,
                    ..
                })
            ) {
                self.advance(); // Consume dot

                match &self.current {
                    Some(Token {
                        kind: TokenKind::Asterisk,
                        span,
                    }) => {
                        let end_span = span.clone();
                        self.advance(); // Consume *
                        Ok(SelectItem::QualifiedWildcard {
                            span: Span::from(identifier.span.start..end_span.end),
                            qualifier: self.get_identifier_text(&identifier),
                        })
                    }
                    _ => {
                        // Regular qualified column
                        self.parse_regular_select_item(Some(identifier))
                    }
                }
            } else {
                // Regular unqualified column
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

    // Handle optional alias
    let alias = if matches!(
        self.current,
        Some(Token {
            kind: TokenKind::Identifier,
            ..
        })
    ) {
        Some(self.get_identifier_text(&self.advance().unwrap()))
    } else {
        None
    };

    Ok(SelectItem::Expression {
        span: expr.span(),
        expr,
        alias,
    })
}
