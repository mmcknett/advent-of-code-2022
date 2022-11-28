#[derive(Debug, PartialEq)]
pub enum Expr {
  Number(i32),
  Op(Box<Expr>, OpCode, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum OpCode {
  Mul,
  Div,
  Add,
  Sub
}