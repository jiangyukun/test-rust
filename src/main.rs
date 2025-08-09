#![allow(warnings)]
mod statement;

use crate::statement::*;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut str = String::new();
    let file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/a.js");
    print!("{file_path}");
    File::open(file_path)
        .unwrap()
        .read_to_string(&mut str)
        .expect("Failed to read file");
    println!("{:#?}", str);

    let mut result: Vec<Word> = vec![];
    let mut i = 0;
    loop {
        // print!("main {i}\n");
        if i >= str.len() {
            break;
        }
        let c = str.chars().nth(i).unwrap();
        match c {
            ' ' => i = i + 1,
            '+' | '-' | '*' | '/' | '%' | '>' | '<' | '|' => {
                result.push(Word::read_operation(&mut i, &str))
            }
            '\r' | '\n' => result.push(Word::read_newline(&mut i, &str)),
            ';' | '=' | '(' | ')' | '{' | '}' | '.' | '!' => {
                result.push(Word::new(&c.to_string(), KeyWord::Control));
                i = i + 1
            }
            '_' | 'a'..='z' | 'A'..='Z' => {
                result.push(Word::read_word(&mut i, &str));
            }
            '0'..='9' => {
                result.push(Word::read_digit(&mut i, &str));
            }
            _ => panic!("Unrecognized character {c}"),
        }
    }
    print!("{:#?}", result);

    let mut word_list = WordList {
        index: 0,
        list: result,
    };

    loop {
        let word = skip_empty(&mut word_list);
        match word {
            Some(word) => match word.content.as_str() {
                "let" | "var" | "const" => {
                    let d = VariableDeclaration::build(&mut word_list);
                    print!("{d:?}");
                }
                "for" => {
                    let d = ForStatement::build(&mut word_list);
                    print!("{d:?}");
                }
                o => {
                    panic!("unsupported {o}")
                }
            },
            None => break,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum KeyWord {
    Empty,
    Let,
    For,
    Variable,
    Digit,
    String,
    Control,
}

#[derive(Debug)]
struct Word {
    content: String,
    key: KeyWord,
}

impl Word {
    fn new(w: &str, key: KeyWord) -> Word {
        Word {
            content: String::from(w),
            key,
        }
    }
    fn get_word(&self) -> &Word {
        &self
    }
    fn read_word(i: &mut usize, source: &str) -> Word {
        let mut c = source.chars().nth(*i).unwrap();
        let mut word = String::new();
        word.push(c);
        loop {
            *i = *i + 1;
            // print!("word {i}\n");
            c = source.chars().nth(*i).unwrap();
            match c {
                '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => {
                    word.push(c);
                }
                _ => {
                    break;
                }
            }
        }
        Word {
            content: word,
            key: KeyWord::Variable,
        }
    }
    fn read_operation(i: &mut usize, source: &str) -> Word {
        let mut c = source.chars().nth(*i).unwrap();
        let mut word = String::new();
        word.push(c);
        *i = *i + 1;
        let next = source.chars().nth(*i);
        match next {
            Some(next) => match next {
                '=' => match c {
                    '+' | '-' | '*' | '/' | '%' => {
                        *i = *i + 1;
                        return Word {
                            content: format!("{c}{next}"),
                            key: KeyWord::Control,
                        };
                    }
                    _ => {}
                },
                '+' | '-' | '>' | '<' | '|' => {
                    if next == c {
                        *i = *i + 1;
                        return Word {
                            content: format!("{c}{next}"),
                            key: KeyWord::Control,
                        };
                    }
                }
                _ => {}
            },
            None => {}
        }
        Word {
            content: format!("{c}"),
            key: KeyWord::Control,
        }
    }
    fn read_newline(i: &mut usize, source: &str) -> Word {
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
        Word {
            key: KeyWord::Control,
            content: "\n".to_string(),
        }
    }
    fn read_digit(i: &mut usize, source: &str) -> Word {
        let mut c = source.chars().nth(*i).unwrap();
        let mut word = String::new();
        word.push(c);
        loop {
            *i = *i + 1;
            // print!("digit {i}\n");
            c = source.chars().nth(*i).unwrap();
            match c {
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
        }
        Word {
            content: word,
            key: KeyWord::Digit,
        }
    }
}
