#![allow(warnings)]
mod statement;
mod express;
mod lex;

use crate::statement::*;
use std::fs::File;
use std::io::Read;
use crate::lex::lex;

fn main() {
    let mut str = String::new();
    let file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/a.js");
    print!("{file_path}");
    File::open(file_path)
        .unwrap()
        .read_to_string(&mut str)
        .expect("Failed to read file");
    println!("{:#?}", str);

    let result = lex(&str);
    print!("{:#?}", result);

    let mut word_list = WordList {
        index: 0,
        list: result,
    };

    loop {
        let word = skip_empty(&mut word_list);
        match word {
            Some(word) => match word.content() {
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
