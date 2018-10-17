use std::marker::PhantomData;
use predef::*;
use env::{ ID, LocalID, PushLocal };
use envs::*;
use super::{ Type, Exp, ErrID, TypeCheck, TypeCheckIter, SetLocal };

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Pattern {
    Var(String, Type),
    Atom(ID<ExpVal>, Vec<Type>),
    Comp(ID<ExpVal>, Vec<Type>, Box<Pattern>),
    Tuple(Vec<Pattern>),
}

impl TypeCheck for Pattern {
    fn type_check(&self, env: &LocalEnvs) -> Result<Type, ErrID> {
        Ok(match self {
            Pattern::Var(_, ty) => ty.clone(),
            Pattern::Atom(id, gs) => env.exp.get(*id).ty(gs),
            Pattern::Comp(id, gs, p) => env.exp.get(*id).ty(gs).call_output(&p.type_check(env)?)?,
            Pattern::Tuple(v) => Type::Tuple(v.type_check(env)?),
        })
    }
}

impl<T: TypeCheck> TypeCheck for (Pattern, T) {
    fn type_check(&self, env: &LocalEnvs) -> Result<Type, ErrID> {
        let (p, e) = self;
        let b = e.type_check(&env.scope(p.bound()))?;
        Ok(func(p.type_check(env)?, b))
    }
}

impl<U: PushLocal<ExpVal>> PushLocal<ExpVal> for (Pattern, U) {
    fn push_local_with_min(&self, ph: PhantomData<ExpVal>, min: usize, amount: usize) -> Self {
        let (p, e) = self;
        (p.clone(), e.push_local_with_min(ph, min + p.bounds(), amount))
    }
}

impl PushLocal<TypeVal> for Pattern {
    fn push_local_with_min(&self, ph: PhantomData<TypeVal>, min: usize, amount: usize) -> Self {
        match self {
            Pattern::Var(n, ty) => Pattern::Var(n.clone(), ty.push_local_with_min(ph, min, amount)),
            Pattern::Atom(id, gs) => Pattern::Atom(*id, gs.push_local_with_min(ph, min, amount)),
            Pattern::Comp(id, gs, p) => Pattern::Comp(*id, gs.push_local_with_min(ph, min, amount), p.push_local_with_min(ph, min, amount)),
            Pattern::Tuple(v) => Pattern::Tuple(v.push_local_with_min(ph, min, amount)),
        }
    }
}

impl<U: SetLocal<Exp>> SetLocal<Exp> for (Pattern, U) {
    fn set_with_min(&self, min: usize, par: &[Exp]) -> Self {
        let (p, e) = self;
        (p.clone(), e.set_with_min(min + p.bounds(), par))
    }
}

impl SetLocal<Type> for Pattern {
    fn set_with_min(&self, min: usize, par: &[Type]) -> Self {
        match self {
            Pattern::Var(n, ty) => Pattern::Var(n.clone(), ty.set_with_min(min, par)),
            Pattern::Atom(id, gs) => Pattern::Atom(*id, gs.set_with_min(min, par)),
            Pattern::Comp(id, gs, p) => Pattern::Comp(*id, gs.set_with_min(min, par), p.set_with_min(min, par)),
            Pattern::Tuple(v) => Pattern::Tuple(v.set_with_min(min, par)),
        }
    }
}

impl Pattern {
    pub fn to_exp(&self, i: usize) -> Exp {
        match self {
            Pattern::Var(_, _) => Exp::Var(LocalID::new(i), vec![]),
            Pattern::Atom(id, gs) => Exp::Var((*id).into(), gs.clone()),
            Pattern::Comp(id, gs, p) => Exp::Call(Box::new(Exp::Var((*id).into(), gs.clone())), Box::new(p.to_exp(i))),
            Pattern::Tuple(v) => {
                let mut i = i;
                Exp::Tuple(v.into_iter().map(|p|{let e = p.to_exp(i); i += p.bounds(); e}).collect())
            }
        }
    }

    pub fn names(&self) -> Vec<String> {
        match self {
            Pattern::Var(n, ty) => vec!(n.clone()),
            Pattern::Atom(_, _) => vec!(),
            Pattern::Comp(_, _, p) => p.names(),
            Pattern::Tuple(ps) => ps.into_iter().flat_map(|p|p.names()).collect(),
        }
    }

    pub fn bound(&self) -> Vec<(&str, ExpVal)> {
        match self {
            Pattern::Var(n, ty) => vec!((&n, ExpVal::new_empty(ty.clone(), 0))),
            Pattern::Atom(_, _) => vec!(),
            Pattern::Comp(_, _, p) => p.bound(),
            Pattern::Tuple(ps) => ps.into_iter().flat_map(|p|p.bound()).collect(),
        }
    }

    pub fn bounds(&self) -> usize {
        match self {
            Pattern::Var(_, _) => 1,
            Pattern::Atom(_, _) => 0,
            Pattern::Comp(_, _, p) => p.bounds(),
            Pattern::Tuple(ps) => ps.into_iter().map(|p|p.bounds()).sum(),
        }
    }

    pub fn match_exp(&self, e: Exp, env: &LocalEnvs) -> Result<Vec<Exp>, ErrID> {
        match &self {
            Pattern::Var(_, ty) => {
                let e_ty = e.type_check(env)?;
                if e_ty == *ty {
                    Ok(vec![e])
                } else {
                    Err(ErrID::TypeMismatch(e_ty, ty.clone()))
                }
            },
            Pattern::Atom(a, gs) => {
                if let Exp::Var(ref id, ref g) = e {
                    if id == a && g == gs {
                        return Ok(vec![]);
                    }
                }

                Err(ErrID::ExpMismatch(e, Exp::Var((*a).into(), gs.clone())))
            }
            Pattern::Comp(c, gs, box p) => {
                if let Exp::Call(box Exp::Var(f, g), box e) = &e {
                    if f == c && g == gs {
                        return p.match_exp(e.clone(), env)
                    }
                }

                Err(ErrID::ExpMismatch(e, Exp::Var((*c).into(), vec![])))
            },
            Pattern::Tuple(ps) => {
                if let Exp::Tuple(es) = &e {
                    if ps.len() == es.len() {
                        return ps.into_iter().zip(es).map(|(p, e)|p.match_exp(e.clone(), env)).fold(Ok(vec![]), |v, r|{let mut v = v?; v.extend(r?); Ok(v)});
                    }
                }

                Err(ErrID::TypeMismatch(self.type_check(env)?, e.type_check(env)?))
            },
        }
    }
}