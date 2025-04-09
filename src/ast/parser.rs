use logos::Lexer;
use super::{grammar::*, tree::Tree, lexer::Token};
use std::fmt::Debug;


pub fn parse<'source>(lexer: Lexer<'source, Token>) -> Tree
where
Token: logos::Logos<'source> + Debug, {
    let lines = make_lines(lexer);
    println!("{:?}", lines);

    
    let tree : Vec<LineEnum> = lines.iter().map(
        |line| parse_line(line)
    ).collect();

    println!("{:?}", tree);

    return Tree{body: Node::Value(ValueEnum::Scalar(1))}
}

fn parse_line(line : &[Token]) -> LineEnum {
    match &line[0] {
        Token::RETURN => LineEnum::Statement( StatementEnum::Return(Box::new(parse_expr(&line[1..]))) ),
        Token::LET => {
            match &line[1] {
                Token::VARIABLE(s) => LineEnum::Statement(StatementEnum::Let(s.clone(), Box::new(parse_expr(&line[2..])))), //no equals sign for now
                _ => panic!("Expected variable name")
            }
        },
        Token::PRINT => todo!(),
        Token::OPEN => todo!(),
        Token::CLOSE => todo!(),
        Token::VARIABLE(_) => todo!(),

        _ => panic!("Unexpected token, expected statement"),
    }
}

fn parse_expr(line : &[Token]) -> ExpressionEnum {
    match &line[0] {
        Token::NUMBER(_) => {
            let op = line.iter().find(|t| matches!(t, Token::OPERATOR(_)));
            match op {
                Some(_) => ExpressionEnum::Operation(parse_opp(line)),
                None => ExpressionEnum::Value(parse_value(line)),
            }
        },
        Token::VARIABLE(_) => {
            let op = line.iter().find(|t| matches!(t, Token::OPERATOR(_)));
            match op {
                Some(_) => ExpressionEnum::Operation(parse_opp(line)),
                None => ExpressionEnum::Value(parse_value(line)),
            }
        },
        Token::OPEN => {
            let end = line.iter().enumerate().find(|(i, t)| **t == Token::CLOSE).expect("missing closing brackets").0;
            let inside = &line[1..end];
            let op = inside.iter().find(|t| matches!(t, Token::OPERATOR(_)));
            match op {
                Some(_) => ExpressionEnum::Operation(parse_opp(inside)),
                None => ExpressionEnum::Value(parse_value(inside)),
            }
        },

        Token::CLOSE => panic!("no matching openning parenthesis"),
        _ => panic!("Unexpected token, expected expression"),
    }
}

fn parse_opp(line : &[Token]) -> OperationEnum {
    todo!();
}

fn parse_value(line : &[Token]) -> ValueEnum {
    match &line[0] {
        Token::NUMBER(v) => ValueEnum::Scalar(*v),
        Token::VARIABLE(s) => ValueEnum::Variable(s.clone()),

        _ => panic!("Unexpected token, expected value"),
    }
}

fn make_lines<'source>(lexer: Lexer<'source, Token>) -> Vec<Vec<Token>>
where
    Token: logos::Logos<'source> + Debug + PartialEq,
{
    let mut lines = vec![];
    let mut current_line = vec![];

    for result in lexer {
        match result {
            Ok(token) => {
                if token == Token::ENDLINE {
                    lines.push(current_line);
                    current_line = vec![];
                } else {
                    current_line.push(token);
                }
            }
            Err(_) => {}
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}