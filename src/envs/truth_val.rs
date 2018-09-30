use predef::*;
use envs::LocalEnvs;
use id::{ Type, Exp, ErrID };

#[derive(Debug, PartialEq, Eq)]
pub struct TruthVal {
    e: Exp,
}

impl TruthVal {
    pub fn new(e: Exp) -> Self {
        TruthVal{ e }
    }

    pub fn get(&self, gen: Vec<Type>, arg: Option<Exp>, env: &LocalEnvs) -> Result<Exp, ErrID> {
        let mut e = self.e.clone();
        if let Some(arg) = arg {
            e = if let Exp::Call(box Exp::Var(f, _), box Exp::Closure(v)) = &e {
                if *f != FORALL_ID { return Err(ErrID::ExpMismatch(Exp::Var(f.clone(), vec![]), Exp::Var(FORALL_ID.into(), vec![]))); }
                v.into_iter()
                    .filter_map(|(p,a)|{let v = p.match_exp(arg.clone(), env).ok()?; Some(a.set(&v))})
                    .next()
                    .ok_or(ErrID::NoMatch(arg.clone()))?
            } else { return Err(ErrID::ExpMismatch(e.clone(), Exp::Var(FORALL_ID.into(), vec![]))); }
        }
        Ok(e)
    }
}
