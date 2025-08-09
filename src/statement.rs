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
    pub fn next1(&mut self) {
        self.index += 1;
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
    pub fn peek_not_empty_n(&self, n: usize) -> Option<&Word> {
        let mut not_empty_c: usize = 0;
        let mut peek_index: usize = 0;
        loop {
            match self.list.get(self.index + peek_index) {
                Some(word) => match word.key {
                    KeyWord::Control => match word.content.as_str() {
                        "\r" | "\n" | " " | "\t" => {
                            peek_index += 1;
                            continue;
                        }
                        _ => {}
                    },
                    _ => {}
                },
                None => return None,
            }
            if not_empty_c == n {
                break;
            }
            not_empty_c += 1;
        }
        self.list.get(self.index + peek_index)
    }
}

pub fn skip_empty(word_list: &mut WordList) -> Option<&Word> {
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
    word_list.current()
}

fn expect(word_list: &mut WordList, s: &str) {
    let word = skip_empty(word_list);
    match word {
        Some(next) => {
            if next.content != s {
                panic!("expect {s}")
            }
        }
        None => panic!("expect {s}"),
    }
    word_list.next();
    skip_empty(word_list);
}

fn expect_not_null(word_list: &mut WordList) -> Word {
    let word = skip_empty(word_list);
    if word.is_none() {
        panic!("expect_not_null is null")
    }
    let content = word.unwrap().content.to_string();
    let key = word.unwrap().key;
    word_list.next();
    skip_empty(word_list);
    Word { content, key }
}

fn expect_array(word_list: &mut WordList, array: Vec<&str>) -> String {
    let word = skip_empty(word_list);
    match word {
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
impl Node for MemberExpression {}

impl VariableDeclaration {
    pub fn build(word_list: &mut WordList) -> Result<VariableDeclaration, String> {
        let kind = expect_array(word_list, vec!["var", "let", "const"]);
        let mut declarations = vec![];
        declarations.push(VariableDeclarator::build(word_list)?);
        loop {
            let c2 = word_list.current();
            match c2 {
                Some(word) => match word.content.as_str() {
                    "," => {
                        word_list.next();
                        declarations.push(VariableDeclarator::build(word_list)?);
                    }
                    "\r" | "\n" => {
                        word_list.next();
                    }
                    _ => break,
                },
                None => break,
            }
        }
        Ok(VariableDeclaration { kind, declarations })
    }
}

impl VariableDeclarator {
    fn build(word_list: &mut WordList) -> Result<VariableDeclarator, String> {
        let id = expect_not_null(word_list).content;

        let mut equal = skip_empty(word_list);
        if equal.is_none() || equal.unwrap().content != "=" {
            return Ok(VariableDeclarator {
                id,
                init: "undefined".to_string(),
            });
        }
        word_list.next();
        let value = expect_not_null(word_list);
        match value.key {
            KeyWord::Variable | KeyWord::Digit | KeyWord::String => Ok(VariableDeclarator {
                id,
                init: value.content.to_string(),
            }),
            _ => {
                panic!("expect VariableDeclarator value is variable")
            }
        }
    }
}

impl ForStatement {
    pub fn build(word_list: &mut WordList) -> Result<ForStatement, String> {
        let init: Box<dyn Node>;
        let test: Box<dyn Node>;
        let update: Box<dyn Node>;
        let body: Box<dyn Node>;
        expect(word_list, "for");
        expect(word_list, "(");
        let word = word_list.current().expect("");
        match word.content.as_str() {
            ";" => {
                init = Box::new(EmptyStatement {});
            }
            "let" | "var" | "const" => {
                init = Box::new(VariableDeclaration::build(word_list)?);
            }
            _ => {
                init = Box::new(VariableDeclarator::build(word_list)?);
            }
        }
        expect(word_list, ";");
        let word1 = word_list.current().expect("");
        match word1.content.as_str() {
            ";" => {
                test = Box::new(EmptyStatement {});
                word_list.next().expect("export ;");
            }
            _ => {
                test = Box::new(BinaryExpression::build(word_list));
            }
        }
        expect(word_list, ";");
        if word_list.current().expect("").content != ")" {
            update = Box::new(UpdateExpression::build(word_list)?);
        } else {
            update = Box::new(EmptyStatement {});
        }
        expect(word_list, ")");
        let word2 = word_list.current().expect("");
        match word2.content.as_str() {
            "{" => {
                body = Box::new(BlockStatement::build(word_list)?);
            }
            ";" => {
                body = Box::new(EmptyStatement {});
                word_list.next();
            }
            _ => {
                panic!("for()loss")
            }
        }
        Ok(ForStatement {
            init,
            test,
            update,
            body,
        })
    }
}

impl AssignmentExpression {
    pub fn build(word_list: &mut WordList) -> Result<AssignmentExpression, String> {
        todo!()
    }
}

impl UpdateExpression {
    pub fn build(word_list: &mut WordList) -> Result<UpdateExpression, String> {
        let operator;
        let argument: Box<dyn Node>;
        let prefix;
        skip_empty(word_list);

        match word_list.current().expect("").key {
            KeyWord::Control => {
                prefix = true;
                operator = expect_not_null(word_list).content;
                argument = Box::new(Identifier {
                    name: expect_not_null(word_list).content,
                });
            }
            _ => {
                prefix = false;
                argument = Box::new(Identifier {
                    name: expect_not_null(word_list).content,
                });
                operator = expect_not_null(word_list).content;
            }
        }

        Ok(UpdateExpression {
            operator,
            argument,
            prefix,
        })
    }
}

impl BlockStatement {
    pub fn build(word_list: &mut WordList) -> Result<BlockStatement, String> {
        let mut body: Vec<Box<dyn Node>> = vec![];
        expect(word_list, "{");

        loop {
            let expression_statement = ExpressionStatement::build(word_list)?;
            if skip_empty(word_list).expect("").content == "}" {
                break;
            }
            body.push(Box::new(expression_statement));
        }

        expect(word_list, "}");
        Ok(BlockStatement { body })
    }
}

impl ExpressionStatement {
    pub fn build(word_list: &mut WordList) -> Result<ExpressionStatement, String> {
        let expression: Box<dyn Node>;
        let current = skip_empty(word_list);
        match current {
            Some(next) => match next.key {
                KeyWord::Control => match next.content.as_str() {
                    "++" | "--" => {
                        expression = Box::new(UpdateExpression::build(word_list)?);
                    }
                    "+" | "-" | "!" => {
                        panic!("not support")
                    }
                    _ => {
                        panic!("not support")
                    }
                },
                KeyWord::Variable => match word_list.peek_not_empty_n(1) {
                    Some(next2) => match next2.key {
                        KeyWord::Control => match next2.content.as_str() {
                            "++" | "--" => {
                                panic!("not support")
                            }
                            "+=" | "-=" | "*=" | "/=" | "%=" | "===" => {
                                panic!("not support")
                            }
                            "." => {
                                expression = Box::new(MemberExpression::build(word_list)?);
                            }
                            "(" => {
                                expression = Box::new(CallExpression::build(word_list)?);
                            }
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

        Ok(ExpressionStatement { expression })
    }
}

impl CallExpression {
    pub fn build(word_list: &mut WordList) -> Result<CallExpression, String> {
        skip_empty(word_list);

        let callee: Box<dyn Node>;
        let arguments: Box<dyn Node>;
        callee = Box::new(Identifier {
            name: word_list.next().unwrap().content.to_string(),
        });
        if word_list.next().expect("").content != "(" {
            panic!("call (loss")
        }
        arguments = ExpressionStatement::build(word_list)?.expression;
        if word_list.next().expect("").content != ")" {
            panic!("call )loss")
        }
        Ok(CallExpression { callee, arguments })
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

fn get_variable_value(word: Option<&Word>) -> Result<String, String> {
    match word {
        Some(word) => match word.key {
            KeyWord::Variable => Ok(word.content.to_string()),
            _ => Err("expect variable".to_string()),
        },
        None => Err("expect variable, but None".to_string()),
    }
}

impl MemberExpression {
    pub fn build(word_list: &mut WordList) -> Result<MemberExpression, String> {
        let object: Box<dyn Node>;
        let property: Box<dyn Node>;

        object = Box::new(Identifier {
            name: get_variable_value(skip_empty(word_list))?,
        });
        word_list.next();
        expect(word_list, ".");

        property = Box::new(Identifier {
            name: get_variable_value(skip_empty(word_list))?,
        });

        Ok(MemberExpression { object, property })
    }
}
