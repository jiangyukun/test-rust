use crate::lex::Token;
use crate::parser::Parser;

#[derive(Debug, PartialEq)]
pub enum Node {
    EmptyStatement {},
    Identity {
        name: String,
    },
    NumericLiteral {
        value: String,
    },
    VariableDeclaration {
        kind: Token,
        declarations: Vec<Box<Node>>,
    },
    VariableDeclarator {
        id: Box<Node>,
        init: Box<Node>,
    },
    AssignmentExpression {
        left: Box<Node>,
        operator: String,
        right: Box<Node>,
    },
    BinaryExpression {
        left: Box<Node>,
        operator: String,
        right: Box<Node>,
    },
    UnaryExpression {
        operator: String,
        prefix: bool,
        argument: Box<Node>,
    },
    UpdateExpression {
        operator: String,
        prefix: bool,
        argument: Box<Node>,
    },
    MemberExpression {
        object: Box<Node>,
        property: Box<Node>,
    },
    ConditionalExpression {
        test: Box<Node>,
        consequent: Box<Node>,
        alternate: Box<Node>,
    },
    CallExpression {
        callee: Box<Node>,
        arguments: Vec<Box<Node>>,
    },
    ForStatement {
        init: Box<Node>,
        test: Box<Node>,
        update: Box<Node>,
        body: Box<Node>,
    },
}

pub fn parse_expression(parser: &mut Parser, min_level: u8) -> Result<Box<Node>, String> {
    let word = parser.current.clone();
    if is_ctrl(&word) {
        let l = get_level(&word)?;
        if let Token::Control(s) = word {
            match s.as_str() {
                "++" => {
                    parser.next();
                    return Ok(Box::new(Node::UpdateExpression {
                        operator: s.to_string(),
                        prefix: true,
                        argument: parse_expression(parser, l + 1)?,
                    }));
                }
                "+" | "-" | "!" | "typeof" => {
                    parser.next();
                    return Ok(Box::new(Node::UnaryExpression {
                        operator: s.to_string(),
                        prefix: true,
                        argument: parse_expression(parser, l + 1)?,
                    }));
                }
                "(" => {
                    parser.next();
                    let express = parse_expression(parser, 1)?;
                    parser.next();
                    if !is_ctrl_word(&parser.current, ")") {
                        return Err("expect )".to_string());
                    }
                    parser.next();
                    return Ok(express);
                }
                _ => {}
            }
        }
        return Err("expect control,".to_string());
    }
    let mut left: Result<Box<Node>, String>;

    if let Token::Variable(s) = word {
        left = ok_box(Node::Identity {
            name: s.to_string(),
        })
    } else if let Token::Digit(d) = word {
        left = ok_box(Node::NumericLiteral {
            value: d.to_string(),
        })
    } else {
        return Err(format!("unsupported parse_express start {word}"));
    }

    loop {
        match &parser.lookahead {
            Token::Control(s) => match s.as_str() {
                ";" | ":" | ")" | "]" | "," => {
                    return left;
                }
                _ => {}
            },
            Token::EOF => return left,
            Token::Variable(_) => return left,
            Token::Digit(_) => return left,
            _ => return left,
        }
        let l = get_level(&parser.lookahead)?;
        if l < min_level {
            break;
        }
        parser.next();
        match &parser.current {
            Token::Control(s) => match s.as_str() {
                "++" | "--" => {
                    return ok_box(Node::UpdateExpression {
                        operator: s.to_string(),
                        prefix: false,
                        argument: left?,
                    });
                }
                "?" => {
                    parser.next();
                    let consequent = parse_expression(parser, l + 1)?;
                    parser.next();
                    if !is_ctrl_word(&parser.current, ":") {
                        return Err("expect :".to_string());
                    }
                    parser.next();
                    let alternate = parse_expression(parser, l + 1)?;
                    return ok_box(Node::ConditionalExpression {
                        test: left?,
                        consequent,
                        alternate,
                    });
                }
                "(" => {
                    parser.next();
                    let mut arguments: Vec<Box<Node>> = vec![];
                    loop {
                        let next = &parser.current;
                        if is_ctrl_word(&next, ")") {
                            break;
                        }
                        let express = parse_expression(parser, 1)?;
                        arguments.push(express);
                        parser.next();
                        let current = &parser.current.clone();
                        if is_ctrl_word(&current, ",") {
                            parser.next();
                        }
                        if is_ctrl_word(&current, ")") {
                            break;
                        }
                    }
                    return ok_box(Node::CallExpression {
                        callee: left?,
                        arguments,
                    });
                }
                _ => {}
            },
            _ => return Err("operator err".to_string()),
        }

        let operator = parser.current.clone();
        match &operator {
            Token::Control(s) => match s.as_str() {
                "=" => {
                    parser.next();
                    let right = parse_expression(parser, l + 1)?;
                    left = ok_box(Node::AssignmentExpression {
                        operator: s.to_string(),
                        left: left?,
                        right,
                    })
                }
                "+" | "-" | "*" | "/" | "%" | ">" | "<" | ">=" | "<=" => {
                    parser.next();
                    let right = parse_expression(parser, l + 1)?;
                    left = ok_box(Node::BinaryExpression {
                        operator: s.to_string(),
                        left: left?,
                        right,
                    })
                }
                _ => {
                    return Err(format!("unsupported operator {:?}", &operator));
                }
            },
            _ => {
                return left;
            }
        }
    }
    left
}

pub fn ok_box(node: Node) -> Result<Box<Node>, String> {
    Ok(Box::new(node))
}

fn get_level(token: &Token) -> Result<u8, String> {
    let d = match token {
        Token::Control(s) => match s.as_str() {
            "." | "[" | "]" | "(" | ")" | "?." => 20,
            "new" => 19,
            "++" | "--" => 17,
            "!" | "~" | "typeof" | "await" | "delete" => 16,
            "**" => 15,
            "*" | "/" | "%" => 14,
            "+" | "-" => 13,
            ">" | ">=" | "<" | "<=" => 11,
            "==" | "!=" | "!==" | "===" => 10,
            "&" => 9,
            "^" => 8,
            "|" => 7,
            "&&" => 6,
            "||" => 5,
            "?" | ":" => 3,
            "=" | "+=" | "-=" | "*=" | "/=" | "%=" => 2,
            "," => 1,
            _ => return Err(format!("get level err {token}")),
        },
        _ => return Err(format!("get level err {token}")),
    };
    Ok(d)
}

pub fn is_ctrl_word(word: &Token, str: &str) -> bool {
    match word {
        Token::Control(s) => {
            if s == str {
                return true;
            }
            false
        }
        _ => false,
    }
}

pub fn is_ctrl(word: &Token) -> bool {
    match word {
        Token::Control(_) => true,
        _ => false,
    }
}

pub fn skip_empty(parser: &mut Parser) -> Token {
    loop {
        match &parser.current {
            Token::Control(next) => match next.as_str() {
                "\r" | "\n" | " " | "\t" => {}
                _ => break,
            },
            _ => break,
        }
        parser.next();
    }
    parser.current.clone()
}

pub fn expect(word: &Token, s: &str) -> Result<(), String> {
    match word {
        Token::Control(next) => {
            if next != s {
                return Err(format!("expect {s}"));
            }
        }
        _ => return Err(format!("expect {s}")),
    }
    Ok(())
}

pub fn expect_keyword(word: &Token, token: Token) -> Result<(), String> {
    if *word == token {
        return Ok(());
    }
    Err(format!("expect keyword {token}"))
}

pub fn expect_keys(word: &Token, list: &Vec<Token>) -> Result<Token, String> {
    for s in list {
        if s == word {
            return Ok(s.clone());
        }
    }
    Err(format!("expect {list:?}"))
}

#[cfg(test)]
mod test {
    use crate::express::Node::*;
    use crate::express::{ok_box, parse_expression};
    use crate::parser::Parser;

    #[test]
    fn test_dot() {
        let mut parser = Parser::new("a.b.c".to_string());
        let node = parse_expression(&mut parser, 1);
        // print!("{node:#?}\n");
        let result = MemberExpression {
            object: Box::new(MemberExpression {
                object: Box::new(Identity {
                    name: "a".to_string(),
                }),
                property: Box::new(Identity {
                    name: "b".to_string(),
                }),
            }),
            property: Box::new(Identity {
                name: "c".to_string(),
            }),
        };
        assert_eq!(node, ok_box(result))
    }

    #[test]
    fn test_operator() {
        let mut parser = Parser::new("c = a + +b + d++".to_string());
        let node = parse_expression(&mut parser, 1);
        print!("{node:#?}\n");
    }

    #[test]
    fn test_call() {
        let mut parser = Parser::new("c = a ? b(d,e,f) : 2+3".to_string());
        let node = parse_expression(&mut parser, 1);
        print!("{node:#?}\n");
    }
}
