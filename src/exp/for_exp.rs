use crate::express::Node::*;
use crate::express::{Node, expect, is_ctrl_word, parse_expression};
use crate::parser::Parser;

pub fn build_for(parser: &mut Parser) -> Result<Node, String> {
    let init: Box<Node>;
    let test: Box<Node>;
    let update: Box<Node>;
    let body: Box<Node>;
    expect(&parser.current, "for")?;
    parser.next();
    expect(&parser.current, "(")?;
    parser.next();
    let part1 = &parser.current;
    if is_ctrl_word(&part1, ";") {
        init = Box::new(EmptyStatement {});
    } else {
        init = Box::new(parse_expression(parser, 0)?);
    }

    parser.next();
    expect(&parser.current, ";")?;
    parser.next();
    let part2 = &parser.current;
    if is_ctrl_word(&part2, ";") {
        test = Box::new(EmptyStatement {});
    } else {
        test = Box::new(parse_expression(parser, 0)?);
    }

    parser.next();
    expect(&parser.current, ";")?;
    parser.next();
    let part3 = &parser.current;
    if is_ctrl_word(&part3, ")") {
        update = Box::new(parse_expression(parser, 0)?);
    } else {
        update = Box::new(EmptyStatement {});
    }

    parser.next();
    expect(&parser.current, ")")?;
    parser.next();
    if is_ctrl_word(&parser.current, "{") {
        body = Box::new(parse_expression(parser, 0)?);
        parser.next();
        expect(&parser.current, "}")?;
    } else if is_ctrl_word(&parser.current, ";") {
        body = Box::new(EmptyStatement {});
    } else {
        return Err("for body error".to_string());
    }
    Ok(ForStatement {
        init,
        test,
        update,
        body,
    })
}

