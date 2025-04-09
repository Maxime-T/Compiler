use logos::Logos;


#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
  #[token("let")]
  LET,
  #[token("print")]
  PRINT,
  #[token(";")]
  ENDLINE,
  #[token("=")]
  EQUALS,
  #[token("+")]
  PLUS,
  #[token("*")]
  MULT,
  #[token("-")]
  SUB,
  #[regex(r"[0-9]+", |lex| lex.slice().parse().ok())]
  NUMBER(i32),
  #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
  VARIABLE(String),
}

