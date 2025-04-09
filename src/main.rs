mod ast;
use std::{fs::File, io::{self, Read}, path::Path};
use ast::{lexer::Token, parser};
use logos::Logos;

fn main() {
    let source = read_file_to_string("src/hello.txt").expect("File not found");

    let lexer = Token::lexer(&source);

    parser::parse(lexer);
}


fn read_file_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}