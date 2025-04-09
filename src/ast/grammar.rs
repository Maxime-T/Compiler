
#[derive(Debug, Clone, PartialEq)]
pub enum ValueEnum {
  Variable(String),
  Scalar(i32)
}

#[derive(Debug, Clone, PartialEq)]
pub enum OperationEnum {
  Add(Box<ExpressionEnum>, Box<ExpressionEnum>),
  Sub(Box<ExpressionEnum>, Box<ExpressionEnum>),
  Mult(Box<ExpressionEnum>, Box<ExpressionEnum>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionEnum {
  Value(ValueEnum),
  Operation(OperationEnum)
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementEnum {
  Return(Box<ExpressionEnum>),
  Print(Box<ExpressionEnum>),
  GoTo(Box<ExpressionEnum>),
  If(Box<ExpressionEnum>, Body, Body),
  Let(String, Box<ExpressionEnum>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineEnum {
  Statement(StatementEnum),
}

pub type Body = Vec<LineEnum>;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
  Value(ValueEnum),
  Expression(ExpressionEnum),
  Statement(StatementEnum),
  Line(LineEnum),
  Body(Body),
}