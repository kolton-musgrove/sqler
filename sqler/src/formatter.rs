// use crate::ast::AST;
use crate::config::Config;
use crate::error::ParseError;

pub fn format_sql(sql: &str, _config: &Config) -> Result<String, ParseError> {
    // Placeholder for now
    Ok(sql.to_string())
}
