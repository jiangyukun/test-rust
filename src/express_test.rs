#[cfg(test)]
mod test {

    use crate::lex::Token;
    use crate::node::Node::*;
    use crate::parser::Parser;

    #[test]
    fn test_dot() {
        let mut parser = Parser::new("a.b.c".to_string());
        let ast = parser.parse();
        assert_eq!(parser.current, Token::EOF);
    }

    #[test]
    fn test_question() {
        let mut parser = Parser::new("a = b ? c ? d : e : f".to_string());
        let ast = parser.parse();
        assert_eq!(parser.current, Token::EOF);
    }

    #[test]
    fn test_operator() {
        let mut parser = Parser::new("c = a + +b + d++".to_string());
        let ast = parser.parse();
        assert_eq!(parser.current, Token::EOF);
    }

    #[test]
    fn test_call() {
        let mut parser = Parser::new("c = a ? b(d,e,f) : 2+3".to_string());
        let ast = parser.parse();
        assert_eq!(parser.current, Token::EOF);
    }
}
