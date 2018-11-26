use LET::eval::ExpVal;
use std::ops::Deref;
use std::rc::Rc;

enum EnvList {
    Empty,
    Extend(String, ExpVal, Env)
}

pub struct Env(Rc<EnvList>);

impl Env {

    pub fn empty() -> Self {
        Env(Rc::new(EnvList::Empty))
    }


    pub fn extend(&self, var: &str , val: ExpVal) -> Self {
        Env(Rc::new(EnvList::Extend(var.to_string(), val, Env(self.0.clone()))))
    }

    pub fn apply(&self, var: &str) -> Option<ExpVal> {
        match self.0.deref() {
            EnvList::Empty => None,
            EnvList::Extend(v, e, t) =>
                if *v == var {
                    Some(e.clone())
                } else {
                    t.apply(var)
                }
        }
    }
}
