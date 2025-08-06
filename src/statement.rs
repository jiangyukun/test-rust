use crate::Word;

struct WordList {
    index: usize,
    list: Vec<Word>,
}

impl<'a> WordList {
    fn new(index: usize, list: Vec<Word>) -> WordList {
        WordList { index, list }
    }
    fn check_next(&self) -> &str {
        self.list.get(self.index + 1).unwrap().get_word()
    }
    fn next(&self) -> &str {
        self.index += 1;
        self.list.get(self.index).unwrap().get_word()
    }
    fn before(&self) {
        self.index -= 1;
    }
}

pub trait Statement<'a> {
    fn build(&mut self, op: &'a WordList);
}

pub struct LetStatement<'a> {
    var: &'a str,
    variable_list: Box<Vec<VariableStatement>>,
}

impl<'a> Statement<'a> for LetStatement<'a> {
    fn build(&mut self, word_list: &'a WordList) {
        let c = word_list.next();
        self.var = c;
        self.variable_list = Box::new(vec![]);
        let mut v = VariableStatement::new();
        v.build(word_list);
        self.variable_list.push(v);
        // variable_list.push();
        // loop {
        //     if op.check_next() == "," {
        //         op.next();
        //         VariableStatement::build(op);
        //         // variable_list.push();
        //     } else {
        //         break;
        //     }
        // }
    }
}

pub struct VariableStatement {
    name: String,
    value: String,
}

impl VariableStatement {
    fn new() -> VariableStatement {
        VariableStatement {
            name: "".to_string(),
            value: "".to_string(),
        }
    }
}

impl<'a> Statement<'a> for VariableStatement {
    fn build(&mut self, op: &'a WordList) {}
}
