use predef::*;
use envs::LocalEnvs;
use id::{ Type, Exp, ErrID, SetLocal };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TruthVal {
    e: Exp,
    gs: usize,
}

impl TruthVal {
    pub fn new(e: Exp, gs: usize) -> Self {
        TruthVal{ e, gs }
    }

    pub fn get(&self, gen: Vec<Type>, arg: Option<Exp>, env: &LocalEnvs) -> Result<Exp, ErrID> {
        if gen.len() != self.gs { return Err(ErrID::GenericAmount(gen.len(), self.gs)); }
        let mut e = self.e.set(&gen);
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
