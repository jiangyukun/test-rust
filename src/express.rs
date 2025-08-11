use crate::lex::Token;
use crate::parser::Parser;

#[derive(Debug)]
pub enum Node {
    EmptyStatement {},
    Identity {
        name: String,
    },
    NumericLiteral {
        value: String,
    },
    VariableDeclaration {
        kind: String,
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

#[cfg(test)]
mod test {
    use crate::express::parse_expression;
    use crate::parser::Parser;

    #[test]
    fn a() {
        let list = vec![
            // "a.b.c",
            // "c = a + +b + d++",
            "c = a ? b(d,e,f) : 2+3",
        ];
        for item in list {
            let mut parser = Parser::new(item.to_string());
            let node = parse_expression(&mut parser, 1);
            print!("{node:#?}\n");
        }
    }
}

pub fn parse_expression(parser: &mut Parser, min_level: u8) -> Result<Node, String> {
    let word = parser.current.clone();
    if is_ctrl(&word) {
        let l = get_level(&word)?;
        if let Token::Control(s) = word {
            match s.as_str() {
                "++" => {
                    parser.next();
                    return Ok(Node::UpdateExpression {
                        operator: s.to_string(),
                        prefix: true,
                        argument: Box::new(parse_expression(parser, l + 1)?),
                    });
                }
                "+" | "-" | "!" | "typeof" => {
                    parser.next();
                    return Ok(Node::UnaryExpression {
                        operator: s.to_string(),
                        prefix: true,
                        argument: Box::new(parse_expression(parser, l + 1)?),
                    });
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
    let mut left: Node;

    if let Token::Variable(s) = word {
        left = Node::Identity {
            name: s.to_string(),
        }
    } else if let Token::Digit(d) = word {
        left = Node::NumericLiteral {
            value: d.to_string(),
        }
    } else {
        return Err("".to_string());
    }
    if is_ctrl_word(&parser.lookahead, ":") {
        return Ok(left);
    }
    if is_ctrl_word(&parser.lookahead, ")") {
        return Ok(left);
    }
    if is_ctrl_word(&parser.lookahead, "]") {
        return Ok(left);
    }
    if is_ctrl_word(&parser.lookahead, ",") {
        return Ok(left);
    }
    loop {
        let operator = &parser.lookahead.clone();
        if *operator == Token::EOF {
            break;
        }
        let l = get_level(&operator)?;
        if l < min_level {
            break;
        }
        parser.next();
        match operator {
            Token::Control(s) => match s.as_str() {
                "++" | "--" => {
                    return Ok(Node::UpdateExpression {
                        operator: s.to_string(),
                        prefix: false,
                        argument: Box::new(left),
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
                    return Ok(Node::ConditionalExpression {
                        test: Box::new(left),
                        consequent: Box::new(consequent),
                        alternate: Box::new(alternate),
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
                        arguments.push(Box::new(express));
                        parser.next();
                        let current = &parser.current.clone();
                        if is_ctrl_word(&current, ",") {
                            parser.next();
                        }
                        if is_ctrl_word(&current, ")") {
                            break;
                        }
                    }
                    return Ok(Node::CallExpression {
                        callee: Box::new(left),
                        arguments,
                    });
                }
                _ => {}
            },
            _ => return Err("operator err".to_string()),
        }

        if is_ctrl_word(&operator, ".") {
            parser.next();
            let right = parse_expression(parser, l + 1)?;
            left = Node::MemberExpression {
                object: Box::new(left),
                property: Box::new(right),
            }
        } else if is_ctrl_word(&operator, "=") {
            parser.next();
            let right = parse_expression(parser, l + 1)?;
            left = Node::AssignmentExpression {
                operator: "=".to_string(),
                left: Box::new(left),
                right: Box::new(right),
            }
        } else if is_ctrl_word(&operator, "+") {
            parser.next();
            let right = parse_expression(parser, l + 1)?;
            left = Node::BinaryExpression {
                operator: "+".to_string(),
                left: Box::new(left),
                right: Box::new(right),
            }
        } else {
            return Err(format!("unsupported operator {operator:?}"));
        }
    }
    Ok(left)
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
            _ => return Err("get level error".to_string()),
        },
        _ => return Err("get level err".to_string()),
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

pub fn expect_keys(word: &Token, list: Vec<&str>) -> Result<String, String> {
    match word {
        Token::Variable(s) => {
            if !list.contains(&s.as_str()) {
                Err(format!("expect {s}"))
            } else {
                Ok(s.to_string())
            }
        }
        _ => Err(format!("expect {list:?}")),
    }
}
