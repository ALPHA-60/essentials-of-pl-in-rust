use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum Exp {
  Const(i32),
  Diff(Rc<Exp>, Rc<Exp>),
  IsZero(Rc<Exp>),
  If(Rc<Exp>, Rc<Exp>, Rc<Exp>),
  Var(String),
  Let(String, Rc<Exp>, Rc<Exp>),
  Proc(String, Rc<Exp>),
  Call(Rc<Exp>, Rc<Exp>)
}
