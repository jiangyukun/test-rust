use crate::lex::{Token, WordList};

#[derive(Debug)]
pub enum Node {
    Identity {
        name: String,
    },
    NumericLiteral {
        value: String,
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
}

#[cfg(test)]
mod test {
    use crate::express::parse_expression;
    use crate::lex::{WordList, lex};

    #[test]
    fn a() {
        let list = vec![
            // "a.b.c",
            // "c = a + +b + d++",
            "c = a ? 1+2 : 2+3",
        ];
        for item in list {
            let result = lex(item);
            let mut word_list = WordList {
                index: 0,
                list: result,
            };
            let node = parse_expression(&mut word_list, 0);
            print!("{node:#?}\n");
        }
    }
}

fn parse_expression(word_list: &mut WordList, min_level: u8) -> Result<Node, String> {
    let word = word_list.current();
    if is_ctrl(&word) {
        let l = get_level(&word)?;
        if let Token::Control(s) = word {
            match s.as_str() {
                "++" => {
                    word_list.next();
                    return Ok(Node::UpdateExpression {
                        operator: s.to_string(),
                        prefix: true,
                        argument: Box::new(parse_expression(word_list, l + 1)?),
                    });
                }
                "+" | "-" | "!" | "typeof" => {
                    word_list.next();
                    return Ok(Node::UnaryExpression {
                        operator: s.to_string(),
                        prefix: true,
                        argument: Box::new(parse_expression(word_list, l + 1)?),
                    });
                }
                _ => {}
            }
        }
        return Err("..".to_string());
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
    if is_ctrl_word(&word_list.peek(), ":") {
        return Ok(left);
    }
    loop {
        let operator = &word_list.peek().clone();
        if *operator == Token::EOF {
            break;
        }
        let l = get_level(operator)?;
        if l < min_level {
            break;
        }
        word_list.next();
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
                    word_list.next();
                    let consequent = parse_expression(word_list, l + 1)?;
                    word_list.next();
                    if !is_ctrl_word(&word_list.current(), ":") {
                        return Err("expect :".to_string());
                    }
                    word_list.next();
                    let alternate = parse_expression(word_list, l + 1)?;
                    return Ok(Node::ConditionalExpression {
                        test: Box::new(left),
                        consequent: Box::new(consequent),
                        alternate: Box::new(alternate),
                    });
                }
                _ => {}
            },
            _ => return Err("operator err".to_string()),
        }
        word_list.next();
        let right = parse_expression(word_list, l + 1)?;

        if is_ctrl_word(operator, ".") {
            left = Node::MemberExpression {
                object: Box::new(left),
                property: Box::new(right),
            }
        } else if is_ctrl_word(operator, "=") {
            left = Node::AssignmentExpression {
                operator: "=".to_string(),
                left: Box::new(left),
                right: Box::new(right),
            }
        } else if is_ctrl_word(operator, "+") {
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
            "?" | ":" => 3,
            "=" | "+=" | "-=" | "*=" | "/=" | "%=" => 2,
            "," => 1,
            _ => return Err("get level error".to_string()),
        },
        _ => return Err("get level err".to_string()),
    };
    Ok(d)
}

fn is_ctrl_word(word: &Token, str: &str) -> bool {
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

fn is_ctrl(word: &Token) -> bool {
    match word {
        Token::Control(_) => true,
        _ => false,
    }
}
