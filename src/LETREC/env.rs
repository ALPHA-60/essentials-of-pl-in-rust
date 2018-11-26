use LETREC::eval::ExpVal;
use LETREC::ast::Exp;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
enum EnvVal {
    Exp(ExpVal),
    Rec(String, Rc<Exp>, Env)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Env(Vec<(String, EnvVal)>);

impl Env {
    pub fn empty() -> Env {
        Env(Vec::new())
    }

    pub fn extend(&mut self, var: String , val: ExpVal) {
        self.0.push((var, EnvVal::Exp(val)));
    }

    pub fn extend_rec(&mut self, proc_name: String , bound_var: String, proc_body: Rc<Exp>) {
        let e = self.clone();
        self.0.push((proc_name, EnvVal::Rec(bound_var, proc_body, e)))
    }

    pub fn pop_last(&mut self) -> () {
        self.0.pop();
    }

    pub fn apply(&self, var: String) -> Option<ExpVal> {
        let mut i = self.0.len() as isize - 1;
        while i != -1 && self.0[i as usize].0 != var {
            i = i - 1;
        };
        if i == -1 {
            None
        } else {
            match &self.0[i as usize].1 {
                EnvVal::Exp(e) => Some(e.clone()),
                EnvVal::Rec(bound_var, proc_body, env) => {
                    let mut env2 = env.clone();
                    env2.extend_rec(var.to_string(), bound_var.to_string(), proc_body.clone());
                    let proc = ExpVal::Proc(bound_var.to_string(), proc_body.clone(), env2);
                    Some(proc)
                }
            }
        }
    }
}
