#[derive(Debug, Clone)]
pub struct Config {
    pub uppercase_keywords: bool,    // uppercase keywords like SELECT, FROM
    pub indent_char: String,         // character to use for indentation
    pub indent_width: usize,         // number of characters per indent level
    pub max_line_length: usize,      // maximum length of a line before wrapping
    pub align_columns: bool,         // align columns in a SELECT clause vertically
    pub line_breaks: LineBreakStyle, // Where to insert line breaks
    pub dialect: SqlDialect,         // SQL dialect to use
}

impl Default for Config {
    fn default() -> Self {
        Self {
            uppercase_keywords: true,
            indent_char: " ".to_string(),
            indent_width: 4,
            max_line_length: 120,
            align_columns: true,
            line_breaks: LineBreakStyle::Always,
            dialect: SqlDialect::Generic,
        }
    }
}

#[derive(Debug, Clone)]
pub enum LineBreakStyle {
    Always,
    Never,
    Inline,
}

#[derive(Debug, Clone)]
pub enum SqlDialect {
    Generic,
    TSql,
}
