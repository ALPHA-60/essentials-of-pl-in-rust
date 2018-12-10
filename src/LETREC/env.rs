use LETREC::eval::ExpVal;
use LETREC::ast::Exp;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
enum EnvList {
    Empty,
    Extend(String, ExpVal, Env),
    ExtendRec(String, String, Rc<Exp>, Env)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Env(Rc<EnvList>);

impl Env {

    pub fn empty() -> Self {
        Env(Rc::new(EnvList::Empty))
    }


    pub fn extend(&self, var: &str , val: ExpVal) -> Self {
        Env(Rc::new(EnvList::Extend(var.to_string(), val, Env(self.0.clone()))))
    }

    pub fn extend_rec(&self, p_name: &str , b_var: &str, p_body: Rc<Exp>) -> Self {
        Env(Rc::new(EnvList::ExtendRec(
                    p_name.to_string(), b_var.to_string(), p_body.clone(), Env(self.0.clone()))))
    }

    pub fn apply(&self, var: &str) -> Option<ExpVal> {
        match self.0.deref() {
            EnvList::Empty => None,
            EnvList::Extend(v, e, t) =>
                if *v == var {
                    Some(e.clone())
                } else {
                    t.apply(var)
                },
            EnvList::ExtendRec(v, b_var, p_body, t) =>
                if *v == var {
                    Some(ExpVal::Proc(b_var.to_string(), (*p_body).clone(), self.clone()))
                } else {
                    t.apply(var)
                }
        }
    }
}
