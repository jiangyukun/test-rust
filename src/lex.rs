pub fn lex(str: &str) -> Vec<Token> {
    let mut result: Vec<Token> = vec![];
    let mut i = 0;
    loop {
        // print!("main {i}\n");
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
    Empty,
    Let,
    For,
    Variable(String),
    Digit(String),
    String,
    Control(String),
    EOF,
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
            }
            None => break
        }
    }
    Token::Digit(word)
}

pub struct WordList {
    pub index: usize,
    pub list: Vec<Token>,
}

impl WordList {
    pub fn current(&mut self) -> Token {
        self.peek_n(0)
    }
    pub fn next(&mut self) -> &Token {
        self.index += 1;
        self.list.get(self.index).unwrap_or_else(|| &Token::EOF)
    }
    pub fn next1(&mut self) {
        self.index += 1;
    }
    pub fn before(&mut self) {
        self.index -= 1
    }
    pub fn peek(&self) -> Token {
        self.peek_n(1)
    }
    pub fn peek_n(&self, n: usize) -> Token {
        self.list
            .get(self.index + n)
            .unwrap_or_else(|| &Token::EOF)
            .clone()
    }
    pub fn peek_not_empty_n(&self, n: usize) -> &Token {
        let mut not_empty_c: usize = 0;
        let mut peek_index: usize = 0;
        loop {
            match self.peek_n(peek_index) {
                Token::Control(s) => match s.as_str() {
                    "\r" | "\n" | "\t" => {
                        peek_index += 1;
                        continue;
                    }
                    _ => {}
                },
                _ => {}
            }
            if not_empty_c == n {
                break;
            }
            peek_index += 1;
            not_empty_c += 1;
        }
        self.list
            .get(self.index + peek_index)
            .unwrap_or_else(|| &Token::EOF)
    }
}
