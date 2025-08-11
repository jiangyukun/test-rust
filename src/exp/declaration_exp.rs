use crate::express::Node::{VariableDeclaration, VariableDeclarator};
use crate::express::{Node, expect_keys, is_ctrl_word};
use crate::lex::Token;
use crate::parser::Parser;

pub fn build(parser: &mut Parser) -> Result<Node, String> {
    let kind = expect_keys(&parser.current, vec!["var", "let", "const"])?;
    let mut declarations = vec![];
    declarations.push(Box::new(build_declarator(parser)?));
    loop {
        let c2 = &parser.current;
        match c2 {
            Token::Control(s) => match s.as_str() {
                "," => {
                    parser.next();
                    declarations.push(Box::new(build_declarator(parser)?));
                }
                "\r" | "\n" => {
                    parser.next();
                }
                _ => break,
            },
            _ => break,
        }
    }
    Ok(VariableDeclaration { kind, declarations })
}

fn build_declarator(parser: &mut Parser) -> Result<Node, String> {
    let id = &parser.current;
    if let Token::Variable(s) = id {
        let id = Box::new(Node::Identity {
            name: s.to_string(),
        });
        parser.next();
        let equal = &parser.current;
        if !is_ctrl_word(equal, "=") {
            return Ok(VariableDeclarator {
                id,
                init: Box::new(Node::Identity {
                    name: "undefined".to_string(),
                }),
            });
        }
        parser.next();
        return Ok(VariableDeclarator {
            id,
            init: Box::new(build_declarator(parser)?),
        });
    }
    Err("".to_string())
}
