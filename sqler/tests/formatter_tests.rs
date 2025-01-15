use sqler::config::Config;
use sqler::formatter::format_sql;

mod tests {
    use super::*;
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
