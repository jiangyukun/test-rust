#![allow(warnings)]
mod statement;

use crate::statement::WordList;
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
            '\r' | '\n' | ';' | '=' | '(' | ')' | '<' | '>' | '+' | '{' | '}' | '.' => {
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
        if word_list.check_next() == None {
            break;
        }
        let word = word_list.check_next();
        match word {
            Some(word) => match &word[..] {
                "let" | "var" | "const" => {
                    statement::LetStatement::build(&mut word_list);
                }
                o => {
                    panic!("unsupported {o}")
                }
            },
            None => break,
        }
    }
}

#[derive(Debug)]
enum KeyWord {
    Let,
    For,
    Variable,
    Digit,
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
