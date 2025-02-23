use sqler::config::{Config, LineBreakStyle, SqlDialect};
use sqler::formatter::format_sql;

mod tests {
    use super::*;
    fn default_config() -> Config {
        Config {
            uppercase_keywords: true,
            indent_width: 4,
            indent_char: " ".to_string(),
            max_line_length: 80,
            align_columns: true,
            line_breaks: LineBreakStyle::Always,
            dialect: SqlDialect::Generic,
        }
    }

    #[test]
    fn test_simple_select() {
        let sql = "select id, name from users";
        let expected = "\
SELECT id,
       name
FROM users";

        let result = format_sql(sql, &default_config()).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_select_with_alias() {
        let sql = "SELECT id AS user_id, name AS user_name FROM users";
        let expected = "\
SELECT id   AS user_id,
       name AS user_name
FROM users";

        let result = format_sql(sql, &default_config()).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_select_with_where_clause() {
        let sql = "SELECT id, name FROM users WHERE id = 1";
        let expected = "\
SELECT id,
       name
FROM users
WHERE id = 1";

        let result = format_sql(sql, &default_config()).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_select_with_group_by() {
        let sql = "SELECT id, name FROM users GROUP BY id";
        let expected = "\
SELECT id,
       name
FROM users
GROUP BY id";

        let result = format_sql(sql, &default_config()).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_select_with_wildcard() {
        let sql = "SELECT * FROM users";
        let expected = "\
SELECT *
FROM users";

        let result = format_sql(sql, &default_config()).unwrap();
        assert_eq!(expected, result);
    }
}
