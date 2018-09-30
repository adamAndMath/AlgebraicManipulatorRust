use std::marker::PhantomData;
use predef::*;
use env::{ ID, LocalID, PushLocal };
use envs::*;
use super::{ Type, Exp, ErrID, TypeCheck, TypeCheckIter, SetLocal };

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Pattern {
    Var(Type),
    Atom(ID<ExpVal>),
    Comp(ID<ExpVal>, Box<Pattern>),
    Tuple(Vec<Pattern>),
}

impl TypeCheck for Pattern {
    fn type_check(&self, env: &LocalEnvs) -> Result<Type, ErrID> {
        Ok(match self {
            Pattern::Var(ty) => ty.clone(),
            Pattern::Atom(id) => env.exp.get(*id)?.ty(),
            Pattern::Comp(id, p) => env.exp.get(*id)?.ty().call_output(&p.type_check(env)?)?,
            Pattern::Tuple(v) => Type::Tuple(v.type_check(env)?),
        })
    }
}

impl<T: TypeCheck> TypeCheck for (Pattern, T) {
    fn type_check(&self, env: &LocalEnvs) -> Result<Type, ErrID> {
        let (p, e) = self;
        let b = e.type_check(&env.scope_anon(p.bound()))?;
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
            Pattern::Var(ty) => Pattern::Var(ty.push_local_with_min(ph, min, amount)),
            Pattern::Atom(id) => Pattern::Atom(*id),
            Pattern::Comp(id, p) => Pattern::Comp(*id, p.push_local_with_min(ph, min, amount)),
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
            Pattern::Var(ty) => Pattern::Var(ty.set_with_min(min, par)),
            Pattern::Atom(id) => Pattern::Atom(*id),
            Pattern::Comp(id, p) => Pattern::Comp(*id, p.set_with_min(min, par)),
            Pattern::Tuple(v) => Pattern::Tuple(v.set_with_min(min, par)),
        }
    }
}

impl Pattern {
    pub fn to_exp(&self, i: usize) -> Exp {
        match self {
            Pattern::Var(_) => Exp::Var(LocalID::new(i), vec![]),
            Pattern::Atom(id) => Exp::Var((*id).into(), vec![]),
            Pattern::Comp(id, p) => Exp::Call(Box::new(Exp::Var((*id).into(), vec![])), Box::new(p.to_exp(i))),
            Pattern::Tuple(v) => {
                let mut i = i;
                Exp::Tuple(v.into_iter().map(|p|{let e = p.to_exp(i); i += p.bounds(); e}).collect())
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

    pub fn bounds(&self) -> usize {
        match self {
            Pattern::Var(_) => 1,
            Pattern::Atom(_) => 0,
            Pattern::Comp(_, p) => p.bounds(),
            Pattern::Tuple(ps) => ps.into_iter().map(|p|p.bounds()).sum(),
        }
    }

    pub fn match_exp(&self, e: Exp, env: &LocalEnvs) -> Result<Vec<Exp>, ErrID> {
        match &self {
            Pattern::Var(ty) => {
                let e_ty = e.type_check(env)?;
                if e_ty == *ty {
                    Ok(vec![e])
                } else {
                    Err(ErrID::TypeMismatch(e_ty, ty.clone()))
                }
            },
            Pattern::Atom(a) => {
                if let Exp::Var(id, _) = e {
                    if id == *a {
                        return Ok(vec![]);
                    }
                }

                Err(ErrID::ExpMismatch(e, Exp::Var((*a).into(), vec![])))
            }
            Pattern::Comp(c, box p) => {
                if let Exp::Call(box Exp::Var(f, _), box e) = &e {
                    if f == c {
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