
mod statement;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut str = String::new();
    File::open("C:/Users/20250713/RustroverProjects/test-rust/src/a.js")
        .unwrap()
        .read_to_string(&mut str)
        .expect("Failed to read file");
    println!("{:#?}", str);

    let mut result: Vec<Word> = vec![];
    let mut i = 0;
    loop {
        print!("main {i}\n");
        if i >= str.len() {
            break;
        }
        let c = str.chars().nth(i).unwrap();
        match c {
            '\r' | '\n' => i = i + 1,
            ' ' => i = i + 1,
            ';' | '=' | '(' | ')' | '<' | '>' | '+' | '{' | '}' | '.' => {
                result.push(Word::new(&c.to_string()));
                i = i + 1
            }
            '_' | 'a'..='z' | 'A'..='Z' => {
                result.push(Word::read_word(&mut i, &str));
            }
            '0'..='9' => {
                result.push(Word::read_digit(&mut i, &str));
            }
            _=> panic!("Unrecognized character {c}")
        }
    }
    print!("{:#?}", result);
}





#[derive(Debug)]
struct Word {
    word: String,
}

impl Word {
    fn new(w: &str) -> Word {
        Word {
            word: String::from(w),
        }
    }
    fn read_word(i: &mut usize, source: &str) -> Word {
        let mut c = source.chars().nth(*i).unwrap();
        let mut word = String::new();
        word.push(c);
        loop {
            *i = *i + 1;
            print!("word {i}\n");
            c = source.chars().nth(*i).unwrap();
            match c {
                '_' => {
                    word.push(c);
                }
                'a'..='z' | 'A'..='Z' => {
                    word.push(c);
                }
                _ => {
                    break;
                }
            }
        }
        Word { word }
    }
    fn read_digit(i: &mut usize, source: &str) -> Word {
        let mut c = source.chars().nth(*i).unwrap();
        let mut word = String::new();
        word.push(c);
        loop {
            *i = *i + 1;
            print!("digit {i}\n");
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
        Word { word }
    }
}

enum KeyWord {
    Let,
    For,
}
