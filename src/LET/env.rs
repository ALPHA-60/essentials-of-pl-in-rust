use LET::eval::ExpVal;

pub struct Env(Vec<(String, ExpVal)>);

impl Env {
    pub fn empty() -> Env {
        Env(Vec::new())
    }
    pub fn extend(&mut self, var: String , val: ExpVal) {
        self.0.push((var, val));
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
            Some(self.0[i as usize].1.clone())
        }
    }
}
