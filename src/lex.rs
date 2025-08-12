use std::fmt::{Display, Formatter};

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

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Control(s) => {
                write!(f, "{}", s)
            }
            Token::Variable(s) => {
                write!(f, "{}", s)
            }
            Token::Digit(s) => {
                write!(f, "{}", s)
            }
            Token::Var => write!(f, "Var"),
            Token::Let => write!(f, "Let"),
            Token::Const => write!(f, "Const"),
            Token::Undefined => write!(f, "Undefined"),
            Token::Null => write!(f, "Null"),
            Token::Await => write!(f, "Await"),
            Token::Async => write!(f, "Async"),
            Token::Function => write!(f, "Function"),
            Token::With => write!(f, "With"),
            Token::If => write!(f, "If"),
            Token::Switch => write!(f, "Switch"),
            Token::Case => write!(f, "Case"),
            Token::Break => write!(f, "Break"),
            Token::Continue => write!(f, "Continue"),
            Token::For => write!(f, "For"),
            Token::While => write!(f, "While"),
            Token::EOF => write!(f, "EOF"),
            _ => {
                write!(f, "token")
            }
        }
    }
}

pub struct Lex {
    input: String,
    pos: usize,
}
impl Lex {
    pub fn new(input: String) -> Self {
        Lex { input, pos: 0 }
    }
    pub fn next(&mut self) -> Token {
        let str = &self.input;
        if self.pos == str.len() {
            return Token::EOF;
        }
        if self.pos > str.len() {
            panic!("end of source");
        }
        loop {
            let c = str.chars().nth(self.pos);
            match c {
                Some(c) => match c {
                    ' ' | '\r' | '\n' => self.pos += 1,
                    '=' | '+' | '-' | '*' | '/' | '%' | '>' | '<' | '|' | '?' | ':' => {
                        return read_operation(&mut self.pos, &str);
                    }
                    ';' | '(' | ')' | '{' | '}' | '.' | '!' | ',' => {
                        self.pos += 1;
                        return Token::Control(c.to_string());
                    }
                    '_' | 'a'..='z' | 'A'..='Z' => return read_word(&mut self.pos, &str),
                    '0'..='9' => return read_digit(&mut self.pos, &str),
                    _ => panic!("Unrecognized character {c}"),
                },
                None => return Token::EOF,
            }
        }
    }
}

fn read_word(i: &mut usize, source: &str) -> Token {
    let c = source.chars().nth(*i).unwrap();
    let mut word = String::new();
    word.push(c);
    loop {
        *i = *i + 1;
        let d = source.chars().nth(*i);
        match d {
            Some(d) => match d {
                '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => word.push(d),
                _ => break,
            },
            None => break,
        }
    }
    match word.as_str() {
        "var" => Token::Var,
        "let" => Token::Let,
        "const" => Token::Const,
        "undefined" => Token::Undefined,
        "null" => Token::Null,
        "await" => Token::Await,
        "async" => Token::Async,
        "function" => Token::Function,
        "with" => Token::With,
        "if" => Token::If,
        "switch" => Token::Switch,
        "case" => Token::Case,
        "break" => Token::Break,
        "continue" => Token::Continue,
        "for" => Token::For,
        "while" => Token::While,
        _ => Token::Variable(word),
    }
}

fn read_operation(i: &mut usize, source: &str) -> Token {
    let c = source.chars().nth(*i).unwrap();
    let mut word = String::new();
    word.push(c);
    loop {
        *i = *i + 1;
        let c = source.chars().nth(*i);
        match c {
            Some(c) => match c {
                '=' | '+' | '-' | '*' | '/' | '%' | '>' | '<' | '|' | '?' | ':' => {
                    word.push(c);
                }
                _ => {
                    break;
                }
            },
            None => break,
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
    let c = source.chars().nth(*i).unwrap();
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

#[cfg(test)]
mod tests {
    use crate::lex::{Lex, Token};

    #[test]
    fn test_token_display() {
        let a = Token::For;
        println!("{a}")
    }

    #[test]
    fn test_keyword() {
        let input = "for(let i = 1; i < 10;i++)++";
        let mut lex = Lex::new(input.to_string());
        assert_eq!(lex.next(), Token::For);
    }

    #[test]
    fn test_lex() {
        let input = " \n\n\nlet\n\n\n a\n\n\n =\n\n\n 1\n\n\n + \n\n\n2\n\n\n";
        let mut lex = Lex::new(input.to_string());

        assert_eq!(lex.next(), Token::Variable("let".to_string()));
        assert_eq!(lex.next(), Token::Variable("a".to_string()));
        assert_eq!(lex.next(), Token::Control("=".to_string()));
        assert_eq!(lex.next(), Token::Digit("1".to_string()));
        assert_eq!(lex.next(), Token::Control("+".to_string()));
        assert_eq!(lex.next(), Token::Digit("2".to_string()));
        assert_eq!(lex.next(), Token::EOF);
    }
}
