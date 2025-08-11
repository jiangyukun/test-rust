pub fn lex(str: &str) -> Vec<Token> {
    let mut result: Vec<Token> = vec![];
    let mut i = 0;
    loop {
        if i >= str.len() {
            return result;
        }
        let c = str.chars().nth(i).unwrap();
        match c {
            ' ' => i = i + 1,
            '=' | '+' | '-' | '*' | '/' | '%' | '>' | '<' | '|' | '?' | ':' => {
                result.push(read_operation(&mut i, &str))
            }
            '\r' | '\n' => result.push(read_newline(&mut i, &str)),
            ';' | '(' | ')' | '{' | '}' | '.' | '!' => {
                result.push(Token::Control(c.to_string()));
                i = i + 1
            }
            '_' | 'a'..='z' | 'A'..='Z' => {
                result.push(read_word(&mut i, &str));
            }
            '0'..='9' => {
                result.push(read_digit(&mut i, &str));
            }
            _ => panic!("Unrecognized character {c}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    LF,
    CR,

    Var,
    Let,
    Const,
    Undefined,
    Null,
    Await,
    Async,
    Function,
    With,
    If,
    Switch,
    Case,
    Break,
    Continue,
    For,
    While,

    Variable(String),
    Digit(String),
    String,
    Control(String),
    EOF,
}

pub struct Lex {
    input: String,
    pos: usize,
}
impl Lex {
    pub fn new(input: String) -> Self {
        Lex { input, pos: 0 }
    }
    pub fn next() -> Token {

    }
}

fn read_word(i: &mut usize, source: &str) -> Token {
    let mut c = source.chars().nth(*i).unwrap();
    let mut word = String::new();
    word.push(c);
    loop {
        *i = *i + 1;
        // print!("word {i}\n");
        let d = source.chars().nth(*i);
        match d {
            Some(d) => match d {
                '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => word.push(c),
                _ => break,
            },
            None => break,
        }
    }
    Token::Variable(word)
}
fn read_operation(i: &mut usize, source: &str) -> Token {
    let mut c = source.chars().nth(*i).unwrap();
    let mut word = String::new();
    word.push(c);
    loop {
        *i = *i + 1;
        // print!("digit {i}\n");
        c = source.chars().nth(*i).unwrap();
        match c {
            '=' | '+' | '-' | '*' | '/' | '%' | '>' | '<' | '|' | '?' | ':' => {
                word.push(c);
            }
            _ => {
                break;
            }
        }
    }
    Token::Control(word)
}
fn read_newline(i: &mut usize, source: &str) -> Token {
    loop {
        *i = *i + 1;
        let c = source.chars().nth(*i);
        match c {
            Some(c) => match c {
                '\r' | '\n' | ' ' | '\t' => {}
                _ => break,
            },
            _ => break,
        }
    }
    Token::Control("\n".to_string())
}

fn read_digit(i: &mut usize, source: &str) -> Token {
    let mut c = source.chars().nth(*i).unwrap();
    let mut word = String::new();
    word.push(c);
    loop {
        *i = *i + 1;
        let c = source.chars().nth(*i);
        match c {
            Some(c) => match c {
                '_' => {
                    word.push(c);
                }
                '0'..='9' => {
                    word.push(c);
                }
                _ => {
                    break;
                }
            },
            None => break,
        }
    }
    Token::Digit(word)
}
