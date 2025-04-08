pub enum ValueEnum {
  Variable(String),
  Scalar(i32)
}

pub enum OperationEnum {
  Add(Box<ExpressionEnum>, Box<ExpressionEnum>),
  Sub(Box<ExpressionEnum>, Box<ExpressionEnum>),
  Mult(Box<ExpressionEnum>, Box<ExpressionEnum>),
}

pub enum ExpressionEnum {
  Value(ValueEnum),
  Operation(OperationEnum)
}

pub enum StatementEnum {
  Return(Box<ExpressionEnum>),
  Print(Box<ExpressionEnum>),
  GoTo(Box<ExpressionEnum>),
  If(Box<ExpressionEnum>, Body, Body),
  Let(String, Box<ExpressionEnum>),
}

pub enum LineEnum {
  Empty,
  Statement(StatementEnum),
}

pub type Body = Vec<LineEnum>;