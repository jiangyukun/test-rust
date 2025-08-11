use crate::lex::Token;

pub struct Parser {
    pub current: Token,
    pub lookahead: Token,

}

impl Parser {
    pub fn new(lex: Lex) -> Parser {
        let parser = Parser {
            current: lex.next(),
            lookahead: lex.next(),

        };

        parser
    }


    
    pub fn next(&mut self) {
        self.index += 1;
        self.current = self
            .list
            .get(self.index)
            .unwrap_or_else(|| &Token::EOF)
            .clone();
        self.lookahead = self
            .list
            .get(self.index + 1)
            .unwrap_or_else(|| &Token::EOF)
            .clone()
    }
    pub fn peek(&self) -> &Token {
        self.peek_not_empty_n(1)
    }
    pub fn peek_n(&self, n: usize) -> Token {
        self.list
            .get(self.index + n)
            .unwrap_or_else(|| &Token::EOF)
            .clone()
    }
    pub fn peek_not_empty_n(&self, n: usize) -> &Token {
        let mut not_empty_c: usize = 0;
        let mut peek_index: usize = 0;
        loop {
            match self.peek_n(peek_index) {
                Token::Control(s) => match s.as_str() {
                    "\r" | "\n" | "\t" => {
                        peek_index += 1;
                        continue;
                    }
                    _ => {}
                },
                _ => {}
            }
            if not_empty_c == n {
                break;
            }
            peek_index += 1;
            not_empty_c += 1;
        }
        self.list
            .get(self.index + peek_index)
            .unwrap_or_else(|| &Token::EOF)
    }
}


#[cfg(test)]
mod test {
    use crate::lex::Token;
    use crate::parser::Parser;

    #[test]
    fn test1() {
        let list = vec![
            Token::LF,
            Token::LF,
            Token::Variable("let".to_string()),
            Token::LF,
            Token::Variable("a".to_string()),
            Token::LF,
            Token::LF,
            Token::Control("=".to_string()),
            Token::LF,
            Token::Variable("b".to_string()),
            Token::Control("+".to_string()),
            Token::Variable("c".to_string()),
        ];
        let parser = Parser::new(list);

        assert_eq!(Token::Variable("let".to_string()), parser.current);
    }
}