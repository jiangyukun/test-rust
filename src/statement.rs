trait Statement<'a> {
    fn build<F>(c: &char, next_char: F) -> Box<Self>
    where
        F: Fn() -> &'a char;
}

struct LetStatement {}

impl<'a> Statement<'a> for LetStatement {
    fn build<F>(c: &char, next_char: F) -> Box<Self>
    where
        F: Fn() -> &'a char,
    {
        todo!()
    }
}
