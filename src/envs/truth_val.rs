use predef::*;
use env::ID;
use envs::Envs;
use id::{ Type, Patterned, Exp, ErrID, SetLocal };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TruthVal {
    e: Exp,
    gs: usize,
}

impl TruthVal {
    pub fn new(e: Exp, gs: usize) -> Self {
        TruthVal{ e, gs }
    }

    pub fn get(&self, id: ID<Self>, gen: Vec<Type>, arg: Option<Exp>, env: &Envs) -> Result<Exp, ErrID> {
        if gen.len() != self.gs { return Err(ErrID::GenericAmount(gen.len(), self.gs)); }
        let mut e = id.move_into(&self.e).set(&gen);
        if let Some(arg) = arg {
            e = if let Exp::Call(box Exp::Var(f, _), box Exp::Closure(v)) = &e {
                if *f != FORALL_ID { return Err(ErrID::ExpMismatch(Exp::Var(f.clone(), vec![]), Exp::Var(FORALL_ID.into(), vec![]))); }
                v.into_iter()
                    .filter_map(|Patterned(p,a)|p.match_exp(arg.clone(), env).ok().map(|v|a.set(&v)))
                    .next()
                    .ok_or(ErrID::NoMatch(arg.clone()))?
            } else { return Err(ErrID::ExpMismatch(e.clone(), Exp::Var(FORALL_ID.into(), vec![]))); }
        }
        Ok(e)
    }
}
