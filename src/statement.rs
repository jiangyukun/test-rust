pub trait Statement<'a> {
    fn build<F, F1>(c: &'a str, next_word: &F, before_word: &F1) -> Self
    where
        F: Fn() -> &'a str,
        F1: Fn();
}

pub struct LetStatement<'a> {
    var: &'a str,
    variable_list: Box<Vec<VariableStatement<'a>>>,
}

impl<'a> Statement<'a> for LetStatement<'a> {
    fn build<F, F1>(c: &'a str, next_word: &F, before_word: &F1) -> Self
    where
        F: Fn() -> &'a str,
        F1: Fn(),
    {
        let mut variable_list = Box::new(vec![]);
        variable_list.push(VariableStatement::build(
            next_word(),
            &next_word,
            &before_word,
        ));
        loop {
            if next_word() == "," {
                variable_list.push(VariableStatement::build(
                    next_word(),
                    &next_word,
                    &before_word,
                ));
            } else {
                before_word();
                break;
            }
        }
        LetStatement {
            var: c,
            variable_list,
        }
    }
}

pub struct VariableStatement<'a> {
    name: &'a str,
    value: &'a str,
}

impl<'a> Statement<'a> for VariableStatement<'a> {
    fn build<F, F1>(c: &'a str, next_word: &F, before_word: &F1) -> Self
    where
        F: Fn() -> &'a str,
        F1: Fn(),
    {
        if next_word() == "=" {
            return VariableStatement {
                name: c,
                value: next_word(),
            };
        }
        before_word();
        VariableStatement {
            name: c,
            value: "undefined",
        }
    }
}
