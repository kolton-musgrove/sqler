use sqler::lexer::Lexer;

mod tests {
    use super::*;

    #[test]
    fn test_simple_select() {
        let sql = "select id, name from users";
        let mut lexer = Lexer::new(sql);
    }
}
