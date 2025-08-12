use crate::lex::{Lex, Token};

pub struct Parser {
    pub current: Token,
    pub lookahead: Token,
    pub list: Vec<Token>,
    lex: Lex,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        let mut lex = Lex::new(input.to_string());
        let current = lex.next();
        let parser = Parser {
            current: current.clone(),
            lookahead: lex.next(),
            list: vec![current],
            lex,
        };

        parser
    }

    pub fn next(&mut self) {
        self.current = self.lookahead.clone();
        self.list.push(self.current.clone());
        self.lookahead = self.lex.next();
    }
}

#[cfg(test)]
mod test {
    use crate::lex::Token;
    use crate::parser::Parser;

    #[test]
    fn test1() {
        let mut parser = Parser::new(" \n let \n a \n = \n b\n ".to_string());

        assert_eq!(Token::Variable("let".to_string()), parser.current);
        parser.next();
        assert_eq!(Token::Variable("a".to_string()), parser.current);
    }
}
