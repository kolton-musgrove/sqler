use crate::ast::*;
use crate::config::Config;
use crate::error::ParseError;
use crate::parser::Parser;

pub fn format_sql(sql: &str, config: &Config) -> Result<String, ParseError> {
    let mut parser = Parser::new(sql);
    let ast = parser.parse_select()?;

    let mut formatter = SqlFormatter::new(config);
    Ok(formatter.format_select(&ast))
}

struct SqlFormatter<'a> {
    config: &'a Config,
    indent_level: usize,
}

impl<'a> SqlFormatter<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            indent_level: 0,
        }
    }

    fn indent(&self) -> String {
        self.config
            .indent_char
            .repeat(self.indent_level * self.config.indent_width)
    }

    fn format_select(&mut self, stmt: &SelectStatement) -> String {
        let mut parts = Vec::new();

        // SELECT clause
        parts.push(format!("{}SELECT", self.indent()));

        self.indent_level += 1;
        // format columns
        let columns = self.format_select_items(&stmt.columns);
        self.indent_level -= 1;

        parts.push(columns);

        // FROM clause
        parts.push(format!(
            "{}FROM {}",
            self.indent(),
            self.format_table_reference(&stmt.from)
        ));

        // WHERE clause
        if let Some(where_clause) = &stmt.where_clause {
            parts.push(format!(
                "{}WHERE {}",
                self.indent(),
                self.format_expression(&where_clause.condition)
            ));
        }

        // GROUP BY clause
        if let Some(group_by) = &stmt.group_by {
            let expressions: Vec<String> = group_by
                .iter()
                .map(|expr| self.format_expression(expr))
                .collect();
            parts.push(format!(
                "{}GROUP BY {}",
                self.indent(),
                expressions.join(", ")
            ));
        }

        parts.join("\n")
    }

    fn format_select_items(&mut self, items: &[SelectItem]) -> String {
        let formatted: Vec<String> = items
            .iter()
            .map(|item| {
                format!(
                    "{}{}",
                    self.indent(),
                    match item {
                        SelectItem::Wildcard { .. } => "*".to_string(),
                        SelectItem::QualifiedWildcard { qualifier, .. } => {
                            format!("{}.*", qualifier)
                        }
                        SelectItem::Expression { expr, alias, .. } => {
                            if let Some(alias_name) = alias {
                                format!("{} AS {}", self.format_expression(expr), alias_name)
                            } else {
                                self.format_expression(expr)
                            }
                        }
                    }
                )
            })
            .collect();

        formatted.join(",\n")
    }

    fn format_expression(&mut self, expr: &Expression) -> String {
        match expr {
            Expression::Column { name, table, .. } => {
                if let Some(table_name) = table {
                    format!("{}.{}", table_name, name)
                } else {
                    name.clone()
                }
            }
            Expression::Literal { value, .. } => match value {
                LiteralValue::String(s) => format!("'{}'", s),
                LiteralValue::Number(n) => n.clone(),
                LiteralValue::Boolean(b) => b.to_string(),
                LiteralValue::Null => "NULL".to_string(),
            },
            Expression::Asterisk { span: _ } => "*".to_string(),
            Expression::BinaryOperation {
                left, op, right, ..
            } => {
                format!(
                    "{} {} {}",
                    self.format_expression(left),
                    op,
                    self.format_expression(right)
                )
            }
            Expression::Function { name, args, .. } => {
                let formatted_args: Vec<String> =
                    args.iter().map(|arg| self.format_expression(arg)).collect();
                format!("{}({})", name, formatted_args.join(", "))
            }
        }
    }

    fn format_table_reference(&mut self, table: &TableReference) -> String {
        let mut result = String::new();

        if let Some(schema) = &table.schema {
            result.push_str(schema);
            result.push('.');
        }

        result.push_str(&table.name);

        if let Some(alias) = &table.alias {
            result.push_str(&format!(" AS {}", alias));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    fn default_config() -> Config {
        Config {
            indent_width: 4,
            indent_char: " ".to_string(),
            max_line_length: 80,
        }
    }

    #[test]
    fn test_simple_select() {
        let sql = "SELECT id, name FROM users";
        let expected = "\
SELECT
    id,
    name
FROM users";

        let result = format_sql(sql, &default_config()).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_select_with_alias() {
        let sql = "SELECT id AS user_id, name AS user_name FROM users";
        let expected = "\
SELECT
    id AS user_id,
    name AS user_name
FROM users";

        let result = format_sql(sql, &default_config()).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_select_with_where_clause() {
        let sql = "SELECT id, name FROM users WHERE id = 1";
        let expected = "\
SELECT
    id,
    name
FROM users
WHERE id = 1";

        let result = format_sql(sql, &default_config()).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_select_with_group_by() {
        let sql = "SELECT id, name FROM users GROUP BY id";
        let expected = "\
SELECT
    id,
    name
FROM users
GROUP BY id";

        let result = format_sql(sql, &default_config()).unwrap();
        assert_eq!(result, expected);
    }
}
