use sqler::ast::SelectItem;
use sqler::lexer::{Token, TokenKind};
use sqler::parser::Parser;

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

    #[test]
    fn test_select_with_alias() {
        let input = "SELECT column1 AS alias1 FROM table1";
        let mut parser = Parser::new(input);
        let result = parser.parse_select().unwrap();

        assert_eq!(result.columns.len(), 1);
        if let SelectItem::Expression { alias, .. } = &result.columns[0] {
            assert_eq!(alias, &Some("alias1".to_string()));
        } else {
            panic!("Expected Expression variant");
        }
    }

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
