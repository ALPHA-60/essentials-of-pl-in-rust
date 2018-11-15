use LET::ast::Exp;
use LET::env::Env;

#[derive(Clone, Debug, PartialEq)]
pub enum ExpVal {
  Num(i32),
  Bool(bool)
}

pub fn value_of(exp : &Exp, env: &mut Env) -> ExpVal {
    match exp {
        &Exp::Const(i) => ExpVal::Num(i),
        &Exp::Var(ref s) => env.apply(s.to_string()).unwrap(),
        &Exp::Diff(box ref exp1, box ref exp2) => 
            match (value_of(exp1, env), value_of(exp2, env)) {
                (ExpVal::Num(num1), ExpVal::Num(num2)) => ExpVal::Num(num1 - num2),
                _ => panic!("difference of non-numbers")
            }
        ,
        &Exp::IsZero(box ref exp1) => match value_of(exp1, env) {
            ExpVal::Num(num1) => ExpVal::Bool(num1 == 0),
            _ => panic!("zero? of non number")
        },
        &Exp::Let(ref var, box ref exp1, box ref exp2) => {
            let v = value_of(exp1, env);
            env.extend(var.to_string(), v);
            let v = value_of(exp2, env);
            env.pop_last();
            v
        }
        &Exp::If(box ref exp1, box ref exp2, box ref exp3) => {
            match value_of(exp1, env) {
                ExpVal::Bool(true) => value_of(exp2, env),
                ExpVal::Bool(false) => value_of(exp3, env),
                _ => panic!("testing a non boolean value")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use LET::parser::parse;
    use super::{value_of, ExpVal};
    use LET::env::Env;

    fn eval(s: &str) -> ExpVal {
        value_of(&parse(s).unwrap(), &mut Env::empty())
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
}
