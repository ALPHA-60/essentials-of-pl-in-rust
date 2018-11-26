use LETREC::ast::Exp;
use LETREC::env::Env;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum ExpVal {
  Num(i32),
  Bool(bool),
  Proc(String, Rc<Exp>, Env)
}

pub fn value_of(exp : &Exp, env: &mut Env) -> ExpVal {
    match exp {
        Exp::Const(i) => ExpVal::Num(*i),
        Exp::Var(s) => match env.apply(s.to_string())  {
            Some(v) => v,
            None => panic!("unknown variable {}", s)
        },
        Exp::Diff(exp1, exp2) =>
            match (value_of(exp1, env), value_of(exp2, env)) {
                (ExpVal::Num(num1), ExpVal::Num(num2)) => ExpVal::Num(num1 - num2),
                _ => panic!("difference of non-numbers")
            }
        ,
        Exp::IsZero(exp1) => match value_of(exp1, env) {
            ExpVal::Num(num1) => ExpVal::Bool(num1 == 0),
            _ => panic!("zero? of non number")
        }
        Exp::Let(var,  exp1, exp2) => {
            let v = value_of(exp1, env);
            env.extend(var.to_string(), v);
            let v = value_of(exp2, env);
            env.pop_last();
            v
        },
        Exp::LetRec(p_name, b_var, p_body, letrec_body) => {
            env.extend_rec(p_name.to_string(), b_var.to_string(), p_body.clone());
            let v = value_of(letrec_body, env);
            env.pop_last();
            v
        }
        Exp::If(exp1,  exp2,  exp3) => {
            match value_of(exp1, env) {
                ExpVal::Bool(true) => value_of(exp2, env),
                ExpVal::Bool(false) => value_of(exp3, env),
                _ => panic!("testing a non boolean value")
            }
        },
        Exp::Proc(var,  body) => ExpVal::Proc(var.to_string(), body.clone(), env.clone()),
        Exp::Call(exp1,  exp2) => {
            match value_of(exp1, env) {
                ExpVal::Proc(var, body, mut saved_env) => {
                    let v = value_of(exp2, env);
                    saved_env.extend(var, v);
                    let v = value_of(&body, &mut saved_env);
                    saved_env.pop_last();
                    v
                }
                _ => panic!("non proc arg")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use LETREC::parser::parse;
    use super::{value_of, ExpVal};
    use LETREC::env::Env;

    fn eval(s: &str) -> ExpVal {
        match parse(s)  {
            Some(x) => value_of(&x, &mut Env::empty()),
            None => panic!("could not parse")
        }
    }

    #[test]
    fn constant() {
        assert_eq!(eval("1"), ExpVal::Num(1));
    }

    #[test]
    fn variable_num() {
        assert_eq!(eval("let x = 3 in x"), ExpVal::Num(3));
    }

    #[test]
    fn variable_bool() {
        assert_eq!(eval("let x = zero?(3) in x"), ExpVal::Bool(false));
    }


    #[test]
    fn diff() {
        assert_eq!(eval("-(5, 3)"), ExpVal::Num(2));
    }

    #[test]
    fn iszero_zero() {
        assert_eq!(eval("zero?(0)"), ExpVal::Bool(true));
    }

    #[test]
    fn iszero_nonzero() {
        assert_eq!(eval("zero?(1)"), ExpVal::Bool(false));
    }


    #[test]
    fn let_nested() {
        assert_eq!(eval("let x = 1 in let y = 2 in -(x,y) "), ExpVal::Num(-1));
    }

    #[test]
    fn let_shadowing() {
        assert_eq!(eval("let x = 1 in let x = 2 in x "), ExpVal::Num(2));
    }

    #[test]
    fn let_popping() {
        assert_eq!(eval("let x = 2 in -(let x = 3 in x, x)"), ExpVal::Num(1));
    }

    #[test]
    fn let_nested_bind() {
        assert_eq!(eval("let x = let y = 3 in y in x"), ExpVal::Num(3));
    }

    #[test]
    fn if_true() {
        assert_eq!(eval("if zero?(0) then 1 else 2"), ExpVal::Num(1));
    }

    #[test]
    fn if_false() {
        assert_eq!(eval("if zero?(1) then 1 else 2"), ExpVal::Num(2));
    }

    #[test]
    fn proc_1() {
        assert_eq!(eval("let f = proc (x) -(x,11) in (f (f 77))"), ExpVal::Num(55));
    }

    #[test]
    fn proc_2() {
        assert_eq!(eval("(proc (f) (f (f 77)) proc (x) -(x, 11))"), ExpVal::Num(55));
    }

    #[test]
    fn proc_3() {
        assert_eq!(eval("let x = 200
                           in let f = proc (z)  -(z, x)
                              in let x = 100
                                 in let g = proc (z) -(z, x)
                                   in -((f 1), (g 1))"), ExpVal::Num(-100));
    }
    #[test]
    fn letrec_double() {
        assert_eq!(eval("
          letrec double(x)
                  = if zero?(x) then 0 else -((double -(x, 1)), -(0, 2))
          in (double 6)"), ExpVal::Num(12));
    }
}
