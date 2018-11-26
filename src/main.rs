
#![feature(box_syntax, box_patterns)]
#[macro_use] extern crate lalrpop_util;

mod LET {
  pub mod ast;
  pub mod eval;
  pub mod parser;
  pub mod env;
}

mod PROC { pub mod ast; pub mod eval; pub mod parser; pub mod env; }

mod LETREC { pub mod ast; pub mod eval; pub mod parser; pub mod env; }


#[derive(Clone)]
enum ExpVal {
  Int(i32),
  Bool(bool)
}

struct Env<'a>(Vec<(&'a str, ExpVal)>);

impl<'a> Env<'a> {
    fn empty() -> Env<'a> {
        Env(Vec::new())
    }

    pub fn extend(&mut self, var: &'a str , val: ExpVal) {
        self.0.push((var, val));
    }

    pub fn pop_last(&mut self) -> () {
        self.0.pop();
    }

    pub fn value_of(&self, var: &'a str) -> Option<ExpVal> {
        let mut i = self.0.len() as isize - 1;
        while i != -1 && self.0[i as usize].0 != var {
            i = i - 1;
        };
        if i == -1 {
            None
        } else {
            Some(self.0[i as usize].1.clone())
        }
    }
}

enum Exp2<'a> {
  Const(i32),
  Diff(&'a Exp2<'a>, &'a Exp2<'a>)
}

fn value_of2<'a, 'b>(exp: &'a Exp2<'a>, env: &mut Env<'b>) -> ExpVal {
    match exp {
        Exp2::Const(i) => ExpVal::Int(*i),
        Exp2::Diff(e1, e2) => match (value_of2(e1, env),  value_of2(e2, env)) {
             (ExpVal::Int(x), ExpVal::Int(y)) => ExpVal::Int(x - y),
             _ => panic!("non integer arguments")
        }
    }
}

enum Exp<'a> {
  Const(i32),
  Diff(Box<Exp<'a>>, Box<Exp<'a>>),
  IsZero(Box<Exp<'a>>),
  If(Box<Exp<'a>>, Box<Exp<'a>>, Box<Exp<'a>>),
  Var(&'a str),
  Let(&'a str, Box<Exp<'a>>, Box<Exp<'a>>)
}

fn value_of<'a>(exp: Exp<'a>, env: &mut Env<'a>) -> ExpVal {
  return match exp {
     Exp::Const(i) => ExpVal::Int(i),

     Exp::Diff(box e1, box e2) =>
         match (value_of(e1, env), value_of(e2,env)) {
             (ExpVal::Int(x), ExpVal::Int(y)) => ExpVal::Int(x - y),
             _ => panic!("non integer arguments")
         }

     Exp::IsZero(box e) => match value_of(e, env) {
         ExpVal::Int(i) => ExpVal::Bool(i == 0),
         _ => panic!("checking zero for a non int")
     },
     Exp::If(box cond, box true_exp, box false_exp) =>
       match value_of(cond, env) {
         ExpVal::Bool(true) => value_of(true_exp, env),
         ExpVal::Bool(false) => value_of(false_exp, env),
         _ => panic!("non boolean condition")
       }
     ,
     Exp::Var(s) => {
         env.value_of(s).unwrap()
     },

     Exp::Let(s, box e1, box e2) => {
       let v = value_of(e1, env);
       env.extend(s, v);
       let v = value_of(e2, env);
       env.pop_last();
       v
     }
  }
}

fn cnst<'a>(i: i32) -> Exp<'a> {
    Exp::Const(i)
}

fn diff<'a>(v1 : Exp<'a>, v2: Exp<'a>) -> Exp<'a> {
    Exp::Diff(Box::new(v1), Box::new(v2))
}

fn is_zero<'a>(v1 : Exp<'a>) -> Exp<'a> {
    Exp::IsZero(Box::new(v1))
}

fn ifthenelse<'a>(cond: Exp<'a>, v1 : Exp<'a>, v2: Exp<'a>) -> Exp<'a> {
    Exp::If(Box::new(cond), Box::new(v1), Box::new(v2))
}

fn letexp<'a>(var: &'a str, v1 : Exp<'a>, v2: Exp<'a>) -> Exp<'a> {
    Exp::Let(var, Box::new(v1), Box::new(v2))
}

fn main() {
    let v = value_of(ifthenelse(is_zero(
            diff(cnst(98), letexp("a", cnst(100), diff(Exp::Var("a"), letexp("a", cnst(3), letexp("b", cnst(2), diff(Exp::Var("a"), Exp::Var("b")))))))),
            cnst(3), cnst(4)), &mut Env::empty());
    match v {
        ExpVal::Int(i) => println!("v: {0}", i),
        ExpVal::Bool(b) => println!("v: {0}", b)
    }

    let exp2 = &Exp2::Diff(&Exp2::Const(2), &Exp2::Const(1));
    let env = &mut Env::empty();
    let v = value_of2(exp2, env);
    match v {
        ExpVal::Int(i) => println!("v: {0}", i),
        ExpVal::Bool(b) => println!("v: {0}", b)
    }
//    println!("{:?}", calc1::ExpressionParser::new().parse("if zero?(- (   let x = 1 in 2 , 33)) then 1 else 2"));

}
