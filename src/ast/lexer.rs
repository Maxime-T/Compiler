use logos::Logos;


#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
  #[token("return")]
  RETURN,
  #[token("let")]
  LET,
  #[token("print")]
  PRINT,
  #[token("(")]
  OPEN,
  #[token(")")]
  CLOSE,
  #[token(";")]
  ENDLINE,
  #[token("=")]
  EQUALS,
  #[token("+", |_lex| Some(Operator::PLUS))]
  #[token("-", |_lex| Some(Operator::SUB))]
  #[token("*", |_lex| Some(Operator::MULT))]
  OPERATOR(Operator),
  #[regex(r"[0-9]+", |lex| lex.slice().parse().ok())]
  NUMBER(i32),
  #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
  VARIABLE(String),
}


#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
  PLUS,
  SUB,
  MULT
}

