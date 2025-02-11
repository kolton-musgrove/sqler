#[derive(Debug, Clone)]
pub struct Config {
    pub indent_char: String,
    pub indent_width: usize,
    pub max_line_length: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            indent_char: " ".to_string(),
            indent_width: 4,
            max_line_length: 120,
        }
    }
}
