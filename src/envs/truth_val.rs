use predef::*;
use id::{ Type, Exp };

#[derive(Debug, PartialEq, Eq)]
pub struct TruthVal {
    e: Exp,
}

impl TruthVal {
    pub fn new(e: Exp) -> Self {
        TruthVal{ e }
    }

    pub fn get(&self, gen: Vec<Type>, par: Vec<Exp>) -> Option<Exp> {
        let mut e = self.e.clone();
        let par = &par[..];
        if !par.is_empty() {
            e = if let Exp::Call(box Exp::Var(f, _), box Exp::Lambda(p, box b)) = &e {
                if *f != FORALL_ID { return None; }
                b.set(par)
            } else { return None; }
        }
        Some(e)
    }
}
