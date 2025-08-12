use crate::exp::declaration_exp::build_let;
use crate::express::Node::*;
use crate::express::{Node, expect, expect_keyword, is_ctrl_word, ok_box, parse_expression};
use crate::lex::Token;
use crate::parser::Parser;

pub fn build_for(parser: &mut Parser) -> Result<Box<Node>, String> {
    let init: Box<Node>;
    let test: Box<Node>;
    let update: Box<Node>;
    let body: Box<Node>;
    expect_keyword(&parser.current, Token::For)?;
    parser.next();
    expect(&parser.current, "(")?;
    parser.next();
    let part1 = &parser.current;
    if *part1 == Token::Let {
        init = build_let(parser)?;
    } else if is_ctrl_word(&part1, ";") {
        init = Box::new(EmptyStatement {});
    } else {
        init = parse_expression(parser, 0)?;
    }

    parser.next();
    expect(&parser.current, ";")?;
    parser.next();
    let part2 = &parser.current;
    if is_ctrl_word(&part2, ";") {
        test = Box::new(EmptyStatement {});
    } else {
        test = parse_expression(parser, 0)?;
    }

    parser.next();
    expect(&parser.current, ";")?;
    parser.next();
    let part3 = &parser.current;
    if is_ctrl_word(&part3, ")") {
        update = parse_expression(parser, 0)?;
    } else {
        update = Box::new(EmptyStatement {});
    }

    parser.next();
    expect(&parser.current, ")")?;
    parser.next();
    if is_ctrl_word(&parser.current, "{") {
        body = parse_expression(parser, 0)?;
        parser.next();
        expect(&parser.current, "}")?;
    } else if is_ctrl_word(&parser.current, ";") {
        body = Box::new(EmptyStatement {});
    } else {
        return Err("for body error".to_string());
    }
    ok_box(ForStatement {
        init,
        test,
        update,
        body,
    })
}

#[cfg(test)]
mod test {
    use crate::exp::for_exp::build_for;
    use crate::parser::Parser;

    #[test]
    fn test_for() {
        let mut parser = Parser::new("for(let i =1; i < 10;i++) {}".to_string());

        let result = build_for(&mut parser);
        println!("{result:#?}");
    }
}
