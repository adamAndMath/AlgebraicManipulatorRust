use predef::*;
use id::{ Type, Exp };

#[derive(Debug)]
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
            e = if let Exp::Call(f, a) = &e {
                if let Exp::Var(f, _) = &**f {
                    if *f != FORALL_ID { return None; }
                } else { return None; }
                if let Exp::Lambda(p, b) = &**a {
                    (&**b).set(par)
                } else { return None; }
            } else { return None; }
        }
        Some(e)
    }
}
