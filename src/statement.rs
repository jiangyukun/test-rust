use crate::{KeyWord, Word};

pub struct WordList {
    pub index: usize,
    pub list: Vec<Word>,
}

impl WordList {
    pub fn current(&mut self) -> Option<&Word> {
        self.peek_n(0)
    }
    pub fn next(&mut self) -> Option<&Word> {
        self.index += 1;
        self.list.get(self.index)
    }
    pub fn before(&mut self) {
        self.index -= 1
    }
    pub fn peek(&self) -> Option<&Word> {
        self.peek_n(1)
    }
    pub fn peek_n(&self, n: usize) -> Option<&Word> {
        self.list.get(self.index + n)
    }
}

pub fn skip_empty(word_list: &mut WordList) {
    loop {
        match word_list.current() {
            Some(next) => match next.key {
                KeyWord::Control => match next.content.as_str() {
                    "\r" | "\n" | " " | "\t" => {}
                    _ => break,
                },
                _ => break,
            },
            None => break,
        }
        word_list.next();
    }
}

fn expect(word_list: &mut WordList, s: &str) {
    skip_empty(word_list);
    match word_list.current() {
        Some(next) => {
            if next.content != s {
                panic!("expect {s}")
            }
        }
        None => panic!("expect {s}"),
    }
    word_list.next();
}


fn expect_array(word_list: &mut WordList, array: Vec<&str>) -> String {
    skip_empty(word_list);
    match word_list.current() {
        Some(next) => {
            if !array.contains(&next.content.as_str()) {
                panic!("expect {array:?}")
            } else {
                let m = next.content.to_string();
                word_list.next();
                m
            }
        }
        None => panic!("expect {array:?}"),
    }
}

fn expect_next(word_list: &mut WordList, s: &str) {
    match word_list.next() {
        Some(next) => {
            if next.content != s {
                panic!("expect {s}")
            }
        }
        None => panic!("expect {s}"),
    }
}

pub trait Node: std::fmt::Debug {}

#[derive(Debug)]
pub struct VariableDeclaration {
    kind: String,
    declarations: Vec<VariableDeclarator>,
}

#[derive(Debug)]
pub struct VariableDeclarator {
    id: String,
    init: String,
}

#[derive(Debug)]
pub struct Identifier {
    name: String,
}

#[derive(Debug)]
pub struct AssignmentExpression {
    operator: String,
    left: Box<dyn Node>,
    right: Box<dyn Node>,
}

#[derive(Debug)]
pub struct BinaryExpression {
    operator: String,
    left: Box<dyn Node>,
    right: Box<dyn Node>,
}

#[derive(Debug)]
pub struct ForStatement {
    init: Box<dyn Node>,
    test: Box<dyn Node>,
    update: Box<dyn Node>,
    body: Box<dyn Node>,
}

#[derive(Debug)]
pub struct UpdateExpression {
    operator: String,
    prefix: bool,
    argument: Box<dyn Node>,
}

#[derive(Debug)]
pub struct BlockStatement {
    body: Vec<Box<dyn Node>>,
}

#[derive(Debug)]
pub struct ExpressionStatement {
    expression: Box<dyn Node>,
}

#[derive(Debug)]
pub struct CallExpression {
    callee: Box<dyn Node>,
    arguments: Box<dyn Node>,
}

#[derive(Debug)]
pub struct MemberExpression {
    object: Box<dyn Node>,
    property: Box<dyn Node>,
}

#[derive(Debug)]
pub struct EmptyStatement {}

impl Node for VariableDeclaration {}
impl Node for VariableDeclarator {}
impl Node for BinaryExpression {}
impl Node for ForStatement {}
impl Node for AssignmentExpression {}
impl Node for Identifier {}
impl Node for UpdateExpression {}
impl Node for BlockStatement {}
impl Node for ExpressionStatement {}
impl Node for EmptyStatement {}
impl Node for CallExpression {}

impl VariableDeclaration {
    pub fn build(word_list: &mut WordList) -> VariableDeclaration {
        let kind = expect_array(word_list, vec!["var", "let", "const"]);
        let mut declarations = vec![];
        declarations.push(VariableDeclarator::build(word_list));
        loop {
            let c2 = word_list.peek();
            match c2 {
                Some(word) => match word.content.as_str() {
                    "," => {
                        word_list.next();
                        declarations.push(VariableDeclarator::build(word_list));
                    }
                    "\r" | "\n" => {
                        word_list.next();
                    }
                    _ => break,
                },
                None => break,
            }
        }
        VariableDeclaration { kind, declarations }
    }
}

impl VariableDeclarator {
    fn build(word_list: &mut WordList) -> VariableDeclarator {
        skip_empty(word_list);

        let id = word_list.next().expect("").content.to_string();
        let mut equal = word_list.next();
        if equal.is_none() {
            return VariableDeclarator {
                id,
                init: "undefined".to_string(),
            };
        }
        if equal.unwrap().content != "=" {
            panic!("expect =")
        }
        let value = word_list.next().expect("export variable value");
        VariableDeclarator {
            id,
            init: value.content.to_string(),
        }
    }
}

impl ForStatement {
    pub fn build(word_list: &mut WordList) -> ForStatement {
        let init: Box<dyn Node>;
        let test: Box<dyn Node>;
        let update: Box<dyn Node>;
        let body: Box<dyn Node>;
        expect(word_list, "for");
        expect(word_list, "(");
        let word = word_list.peek().expect("");
        match word.content.as_str() {
            ";" => {
                init = Box::new(EmptyStatement {});
            }
            "let" | "var" | "const" => {
                init = Box::new(VariableDeclaration::build(word_list));
            }
            _ => {
                init = Box::new(VariableDeclarator::build(word_list));
            }
        }
        expect(word_list, ";");
        let word1 = word_list.peek().expect("");
        match word1.content.as_str() {
            ";" => {
                test = Box::new(EmptyStatement {});
                word_list.next().expect("export ;");
            }
            _ => {
                test = Box::new(BinaryExpression::build(word_list));
            }
        }
        if word_list.next().expect("").content != ";" {
            panic!("expect ;")
        }
        if word_list.peek().expect("").content != ")" {
            update = Box::new(UpdateExpression::build(word_list));
        } else {
            update = Box::new(EmptyStatement {});
        }
        word_list.next().expect(")");

        let word2 = word_list.peek().expect("expect for body");
        match word2.content.as_str() {
            "{" => {
                body = Box::new(BlockStatement::build(word_list));
            }
            ";" => {
                body = Box::new(EmptyStatement {});
                word_list.next();
            }
            _ => {
                panic!("for()loss")
            }
        }
        ForStatement {
            init,
            test,
            update,
            body,
        }
    }
}

impl AssignmentExpression {
    pub fn build(word_list: &mut WordList) -> AssignmentExpression {
        todo!()
    }
}

impl UpdateExpression {
    pub fn build(word_list: &mut WordList) -> UpdateExpression {
        skip_empty(word_list);

        let operator;
        let argument: Box<dyn Node>;
        let prefix;

        match word_list.peek().expect("").key {
            KeyWord::Control => {
                prefix = true;
                operator = word_list.next().expect("").content.to_string();
                argument = Box::new(Identifier {
                    name: word_list.next().expect("").content.to_string(),
                });
            }
            _ => {
                prefix = false;
                argument = Box::new(Identifier {
                    name: word_list.next().expect("").content.to_string(),
                });
                operator = word_list.next().expect("").content.to_string();
            }
        }

        UpdateExpression {
            operator,
            argument,
            prefix,
        }
    }
}

impl BlockStatement {
    pub fn build(word_list: &mut WordList) -> BlockStatement {
        skip_empty(word_list);

        let mut body: Vec<Box<dyn Node>> = vec![];
        if word_list.next().expect("").content != "{" {
            panic!("expect {{")
        }

        loop {
            let expression_statement = ExpressionStatement::build(word_list);
            if word_list.peek().expect("").content == "}" {
                break;
            }
            body.push(Box::new(expression_statement));
        }

        if word_list.next().expect("").content != "}" {
            panic!("expect }}")
        }
        BlockStatement { body }
    }
}

impl ExpressionStatement {
    pub fn build(word_list: &mut WordList) -> ExpressionStatement {
        skip_empty(word_list);

        let expression: Box<dyn Node>;

        match word_list.peek() {
            Some(next) => match next.key {
                KeyWord::Control => match next.content.as_str() {
                    "++" | "--" => {
                        expression = Box::new(UpdateExpression::build(word_list));
                    }
                    "+" | "-" | "!" => {
                        panic!("not support")
                    }
                    _ => {
                        panic!("not support")
                    }
                },
                KeyWord::Variable => match word_list.peek_n(2) {
                    Some(next2) => match next2.key {
                        KeyWord::Control => match next2.content.as_str() {
                            "++" | "--" => {
                                panic!("not support")
                            }
                            "+=" | "-=" | "*=" | "/=" | "%=" | "===" => {
                                panic!("not support")
                            }
                            "\n" => match word_list.peek_n(3) {
                                Some(next3) => match next3.key {
                                    KeyWord::Variable => {
                                        expression = Box::new(Identifier {
                                            name: word_list.next().unwrap().content.to_string(),
                                        });
                                    }
                                    _ => {
                                        panic!("not support")
                                    }
                                },
                                _ => panic!("not support"),
                            },
                            _ => {
                                panic!("not support")
                            }
                        },
                        KeyWord::Variable => {
                            panic!("syn error")
                        }
                        _ => {
                            panic!("not support")
                        }
                    },
                    None => {
                        panic!("not support")
                    }
                },
                _ => {
                    panic!("not support")
                }
            },
            None => {
                panic!("syn error")
            }
        }

        ExpressionStatement { expression }
    }
}

impl CallExpression {
    pub fn build(word_list: &mut WordList) -> CallExpression {
        skip_empty(word_list);

        let callee: Box<dyn Node>;
        let arguments: Box<dyn Node>;
        callee = Box::new(Identifier {
            name: word_list.next().unwrap().content.to_string(),
        });
        if word_list.next().expect("").content != "(" {
            panic!("call (loss")
        }
        arguments = ExpressionStatement::build(word_list).expression;
        if word_list.next().expect("").content != ")" {
            panic!("call )loss")
        }
        CallExpression { callee, arguments }
    }
}

impl BinaryExpression {
    pub fn build(word_list: &mut WordList) -> BinaryExpression {
        skip_empty(word_list);

        let left: Box<dyn Node>;
        let right: Box<dyn Node>;

        left = Box::new(Identifier {
            name: word_list.next().expect("").content.to_string(),
        });
        let operator = word_list.next().expect("").content.to_string();
        right = Box::new(Identifier {
            name: word_list.next().expect("").content.to_string(),
        });

        BinaryExpression {
            left,
            right,
            operator,
        }
    }
}
