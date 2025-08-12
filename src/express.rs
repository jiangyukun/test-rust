use crate::lex::Token;
use crate::node::Node;
use crate::parser::Parser;

pub fn parse_expression(parser: &mut Parser, min_level: u8) -> Result<Box<Node>, String> {
    let word = parser.current.clone();
    if let Token::Control(s) = word {
        let l = get_level(&parser.current)?;
        return match s.as_str() {
            "++" => {
                parser.next();
                Ok(Box::new(Node::UpdateExpression {
                    operator: s.to_string(),
                    prefix: true,
                    argument: parse_expression(parser, l + 1)?,
                }))
            }
            "+" | "-" | "!" | "typeof" => {
                parser.next();
                Ok(Box::new(Node::UnaryExpression {
                    operator: s.to_string(),
                    prefix: true,
                    argument: parse_expression(parser, l + 1)?,
                }))
            }
            "(" => {
                parser.next();
                let express = parse_expression(parser, 1)?;
                if !is_ctrl_word(&parser.current, ")") {
                    return Err("expect )".to_string());
                }
                parser.next();
                Ok(express)
            }
            _ => Err("expect control,".to_string()),
        };
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

    parser.next();
    loop {
        let operator = parser.current.clone();
        match &operator {
            Token::Control(s) => match s.as_str() {
                ";" | ":" | ")" | "]" | "," => break,
                _ => {}
            },
            Token::EOF => break,
            Token::Variable(_) => return Err("syntax error:".to_string()),
            Token::Digit(_) => return Err("syntax error:".to_string()),
            _ => break,
        }
        let l = get_level(&parser.current)?;
        if l < min_level {
            break;
        }

        match &operator {
            Token::Control(s) => match s.as_str() {
                "++" | "--" => {
                    parser.next();
                    return ok_box(Node::UpdateExpression {
                        operator: s.to_string(),
                        prefix: false,
                        argument: left?,
                    });
                }
                "?" => {
                    parser.next();
                    let consequent = parse_expression(parser, l)?;
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
                        let current = &parser.current.clone();
                        if is_ctrl_word(&current, ",") {
                            parser.next();
                        }
                        if is_ctrl_word(&current, ")") {
                            parser.next();
                            break;
                        }
                    }
                    return ok_box(Node::CallExpression {
                        callee: left?,
                        arguments,
                    });
                }
                "=" => {
                    parser.next();
                    let right = parse_expression(parser, l + 1)?;
                    left = ok_box(Node::AssignmentExpression {
                        operator: s.to_string(),
                        left: left?,
                        right,
                    })
                }
                "." => {
                    parser.next();
                    let right = parse_expression(parser, l + 1)?;
                    left = ok_box(Node::MemberExpression {
                        object: left?,
                        property: right,
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
                break;
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
            "." | "[" | "(" | "?." => 20,
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
            "?" => 3,
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
