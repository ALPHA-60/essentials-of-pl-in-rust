#[derive(Debug)]
pub enum Exp {
  Const(i32),
  Diff(Box<Exp>, Box<Exp>),
  IsZero(Box<Exp>),
  If(Box<Exp>, Box<Exp>, Box<Exp>),
  Var(String),
  Let(String, Box<Exp>, Box<Exp>)
}
