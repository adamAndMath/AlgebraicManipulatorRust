use predef::*;
use env::{ ID, LocalID };
use envs::*;
use super::{ Type, Exp };

#[derive(Debug, PartialEq, Clone)]
pub enum Pattern {
    Var(Type),
    Atom(ID<ExpVal>),
    Comp(ID<ExpVal>, Box<Pattern>),
    Tuple(Vec<Pattern>),
}

impl Pattern {
    pub fn type_check(&self, env: &LocalEnvs) -> Option<Type> {
        match self {
            Pattern::Var(ty) => Some(ty.clone()),
            Pattern::Atom(id) => env.exp.get(*id).map(|v|v.ty()),
            Pattern::Comp(id, p) => {
                let f = env.exp.get(*id)?;
                let t = p.type_check(env)?;

                let (p, b) = get_fn_types(f.ty())?;
                if p != t { return None }
                Some(b)
            },
            Pattern::Tuple(v) => Some(Type::Tuple(v.into_iter().map(|p|p.type_check(env)).collect::<Option<_>>()?)),
        }
    }

    pub fn to_exp(&self, i: usize) -> Exp {
        match self {
            Pattern::Var(ty) => Exp::Var(LocalID::new(i), vec![]),
            Pattern::Atom(id) => Exp::Var((*id).into(), vec![]),
            Pattern::Comp(id, p) => Exp::Call(Box::new(Exp::Var((*id).into(), vec![])), Box::new(p.to_exp(i))),
            Pattern::Tuple(v) => {
                let mut i = i;
                Exp::Tuple(v.into_iter().map(|p|{let e = p.to_exp(i); i += p.bound().len(); e}).collect())
            }
        }
    }

    pub fn bound(&self) -> Vec<ExpVal> {
        match self {
            Pattern::Var(ty) => vec!(ExpVal::new_empty(ty.clone(), 0)),
            Pattern::Atom(_) => vec!(),
            Pattern::Comp(_, p) => p.bound(),
            Pattern::Tuple(ps) => ps.into_iter().flat_map(|p|p.bound()).collect(),
        }
    }

    pub fn match_exp(&self, e: Exp, env: &LocalEnvs) -> Option<Vec<Exp>> {
        match &self {
            Pattern::Var(ty) =>
                if e.type_check(env)? == *ty {
                    Some(vec![e])
                } else {None},
            Pattern::Atom(a) =>
                if let Exp::Var(id, _) = e {
                    if id == *a {
                        Some(vec![])
                    } else {None}
                } else {None},
            Pattern::Comp(c, box p) =>
                if let Exp::Call(box Exp::Var(f, _), box e) = e {
                    if f == *c {
                        p.match_exp(e, env)
                    } else {None}
                } else {None},
            Pattern::Tuple(ps) =>
                if let Exp::Tuple(es) = e {
                    if ps.len() == es.len() {
                        ps.into_iter().zip(es).map(|(p, e)|p.match_exp(e, env)).fold(Some(vec![]), |v, r|{let mut v = v?; v.extend(r?); Some(v)})
                    } else {None}
                } else {None},
        }
    }
}