use crate::Word;

pub struct WordList {
    pub index: usize,
    pub list: Vec<Word>,
}

impl WordList {
    pub fn next(&mut self) -> Option<String> {
        self.index += 1;
        if self.index - 1 >= self.list.len() {
            return None;
        }
        Some(
            self.list
                .get(self.index - 1)
                .unwrap()
                .get_word()
                .to_string(),
        )
    }

    pub fn before(&mut self) {
        self.index -= 1
    }

    pub fn check_next(&self) -> Option<String> {
        if self.index >= self.list.len() {
            return None;
        }
        Some(self.list.get(self.index).unwrap().get_word().to_string())
    }
}

pub struct LetStatement {
    var: String,
    variable_list: Vec<VariableStatement>,
}

impl LetStatement {
    pub fn build(word_list: &mut WordList) {
        let c = word_list.next().expect("expect let or var or const");

        let mut let_statement = LetStatement {
            var: c,
            variable_list: vec![],
        };

        VariableStatement::build(&mut let_statement, word_list);
    }
}

pub struct VariableStatement {
    name: String,
    value: String,
}

impl VariableStatement {
    fn build(let_statement: &mut LetStatement, word_list: &mut WordList) {
        loop {
            let c = word_list.next().expect("export var name");

            let v = VariableStatement {
                name: c,
                value: "".to_string(),
            };
            let_statement.variable_list.push(v);

            let c2 = word_list.next();
            match c2 {
                Some(word) => match &word[..] {
                    "=" => {
                        let_statement.variable_list.last_mut().unwrap().value =
                            word_list.next().expect("= export value");
                    }
                    "," => {
                        word_list.next();
                    }
                    "\r" | "\n" => {
                        word_list.next();
                    }
                    _ => {
                        panic!("parse error")
                    }
                },
                None => break,
            }
        }
    }
}
