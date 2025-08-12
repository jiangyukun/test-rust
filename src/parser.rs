use crate::exp::declaration_exp::build_let;
use crate::exp::for_exp::build_for;
use crate::express::{parse_expression};
use crate::lex::{Lex, Token};
use crate::node::Node;

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

    pub fn parse(&mut self) -> Result<Vec<Box<Node>>, String> {
        let mut ast = vec![];
        loop {
            match self.current {
                Token::EOF => break,
                Token::Var | Token::Let | Token::Const => {
                    ast.push(build_let(self)?);
                }
                Token::For => {
                    ast.push(build_for(self)?);
                }

                _ => {
                    ast.push(parse_expression(self, 1)?);
                }
            }
        }
        Ok(ast)
    }
}

#[cfg(test)]
mod parser_test {
    use crate::lex::Token;
    use crate::parser::Parser;

    #[test]
    fn test1() {
        let mut parser = Parser::new(" \n let \n a \n = \n b\n ".to_string());

        assert_eq!(Token::Let, parser.current);
        parser.next();
        assert_eq!(Token::Variable("a".to_string()), parser.current);
    }
}
