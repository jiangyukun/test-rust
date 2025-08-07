use crate::{KeyWord, Word};

pub struct WordList {
    pub index: usize,
    pub list: Vec<Word>,
}

impl WordList {
    pub fn next(&mut self) -> Option<&Word> {
        self.index += 1;
        if self.index >= self.list.len() {
            return None;
        }
        Some(self.list.get(self.index).unwrap().get_word())
    }

    pub fn before(&mut self) {
        self.index -= 1
    }

    pub fn check_next(&self) -> Option<&Word> {
        if self.index + 1 >= self.list.len() {
            return None;
        }
        Some(self.list.get(self.index + 1).unwrap().get_word())
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
        let kind = word_list
            .next()
            .expect("expect let or var or const")
            .content
            .to_string();

        let mut declarations = vec![];

        declarations.push(VariableDeclarator::build(word_list));
        loop {
            let c2 = word_list.check_next();
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
        word_list.next().expect("export for");
        word_list.next().expect("(");
        let word = word_list.check_next().expect("");
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
        if word_list.next().expect("").content != ";" {
            panic!("expect ;")
        }
        let word1 = word_list.check_next().expect("");
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
        if word_list.check_next().expect("").content != ")" {
            update = Box::new(UpdateExpression::build(word_list));
        } else {
            update = Box::new(EmptyStatement {});
        }
        word_list.next().expect(")");

        let word2 = word_list.check_next().expect("expect for body");
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
        let operator;
        let argument: Box<dyn Node>;
        let prefix;

        match word_list.check_next().expect("").key {
            KeyWord::Control => {
                prefix = true;
                let op1 = word_list.next().expect("").content.to_string();
                let op2 = word_list.next().expect("").content.to_string();
                operator = format!("{op1}{op2}");
                argument = Box::new(Identifier {
                    name: word_list.next().expect("").content.to_string(),
                });
            }
            _ => {
                prefix = false;
                argument = Box::new(Identifier {
                    name: word_list.next().expect("").content.to_string(),
                });
                let op1 = word_list.next().expect("").content.to_string();
                let op2 = word_list.next().expect("").content.to_string();
                operator = format!("{op1}{op2}");
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
        let mut body: Vec<Box<dyn Node>> = vec![];
        if word_list.next().expect("").content != "{" {
            panic!("expect {{")
        }

        loop {
            let expression_statement = ExpressionStatement::build(word_list);
            if word_list.check_next().expect("").content == "}" {
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
        let expression: Box<dyn Node>;

        ExpressionStatement { expression }
    }
}

impl CallExpression {
    pub fn build(word_list: &mut WordList) -> CallExpression {
        let callee: Box<dyn Node>;
        let arguments: Box<dyn Node>;



        CallExpression {
            callee,
            arguments,
        }
    }
}

impl BinaryExpression {
    pub fn build(word_list: &mut WordList) -> BinaryExpression {
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
