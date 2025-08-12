use crate::express::Node::{VariableDeclaration, VariableDeclarator};
use crate::express::{Node, expect_keys, is_ctrl_word, parse_expression};
use crate::lex::Token;
use crate::parser::Parser;

pub fn build_let(parser: &mut Parser) -> Result<Box<Node>, String> {
    let kind = expect_keys(&parser.current, &vec![Token::Var, Token::Let])?;
    parser.next();
    let mut declarations = vec![];
    declarations.push(build_declarator(parser)?);
    loop {
        let c2 = &parser.current;
        match c2 {
            Token::Control(s) => match s.as_str() {
                "," => {
                    parser.next();
                    declarations.push(build_declarator(parser)?);
                }
                "\r" | "\n" => {
                    parser.next();
                }
                _ => break,
            },
            _ => break,
        }
    }
    Ok(Box::new(VariableDeclaration { kind, declarations }))
}

fn build_declarator(parser: &mut Parser) -> Result<Box<Node>, String> {
    let id = &parser.current;
    if let Token::Variable(s) = id {
        let id = Box::new(Node::Identity {
            name: s.to_string(),
        });
        parser.next();
        let equal = &parser.current;
        if !is_ctrl_word(equal, "=") {
            return Ok(Box::new(VariableDeclarator {
                id,
                init: Box::new(Node::Identity {
                    name: "undefined".to_string(),
                }),
            }));
        }
        parser.next();
        return Ok(Box::new(VariableDeclarator {
            id,
            init: parse_expression(parser, 1)?,
        }));
    }
    Err(format!("expect Variable, find {id}"))
}

#[cfg(test)]
mod test_let {
    use super::*;
    //
    // #[test]
    // fn test() {
    //     let mut parser = Parser::new("let a = 1".to_string());
    //
    //     let result = build_let(&mut parser);
    //     println!("{result:#?}");
    // }
    //
    // #[test]
    // fn test_express() {
    //     let mut parser = Parser::new("let a = 1 + 2".to_string());
    //
    //     let result = build_let(&mut parser);
    //     println!("{result:#?}");
    // }

    #[test]
    fn test_comma() {
        let mut parser = Parser::new("let a = 3, b = 2".to_string());

        let result = build_let(&mut parser);
        println!("{result:#?}");
    }
}