use predef::*;
use env::{ ID, PushID };
use envs::*;
use super::{ Type, Exp, ErrID, TypeCheck, TypeCheckIter, SetLocal };

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Patterned<T>(pub Pattern, pub T);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Pattern {
    Var(Type),
    Atom(ID<ExpVal>, Vec<Type>),
    Comp(ID<ExpVal>, Vec<Type>, Box<Pattern>),
    Tuple(Vec<Pattern>),
}

impl TypeCheck for Pattern {
    fn type_check(&self, env: &Envs) -> Result<Type, ErrID> {
        Ok(match self {
            Pattern::Var(ty) => ty.clone(),
            Pattern::Atom(id, gs) => {
                let ty = env.exp[*id].ty(*id, gs);
                match ty {
                    Type::Gen(ty_id, _) if env.ty[ty_id].contains_atom(ty_id, id) => ty,
                    ty => return Err(ErrID::NotAtomic((*id).into(), ty)),
                }
            },
            Pattern::Comp(id, gs, p) => {
                let ty = env.exp[*id].ty(*id, gs).call_output(&p.type_check(env)?)?;
                match ty {
                    Type::Gen(ty_id, _) if env.ty[ty_id].contains_comp(ty_id, id) => ty,
                    ty => return Err(ErrID::NotAtomic((*id).into(), ty)),
                }
            },
            Pattern::Tuple(v) => Type::Tuple(v.type_check(env)?),
        })
    }
}

impl<T: TypeCheck> TypeCheck for Patterned<T> {
    fn type_check(&self, env: &Envs) -> Result<Type, ErrID> {
        let Patterned(p, e) = self;
        let b = e.type_check(&env.scope_exp(p.bound()))?;
        Ok(func(p.type_check(&env.scope_exp(vec![]))?, b).pop_id(1).unwrap())
    }
}

impl<T: PushID> PushID for Patterned<T> {
    fn push_id_with_min(&self, min: usize, amount: usize) -> Self {
        let Patterned(p, e) = self;
        Patterned(p.push_id_with_min(min + 1, amount), e.push_id_with_min(min + 1, amount))
    }

    fn pop_id_with_min(&self, min: usize, amount: usize) -> Option<Self> {
        let Patterned(p, e) = self;
        Some(Patterned(p.pop_id_with_min(min + 1, amount)?, e.pop_id_with_min(min + 1, amount)?))
    }
}

impl PushID for Pattern {
    fn push_id_with_min(&self, min: usize, amount: usize) -> Self {
        match self {
            Pattern::Var(ty) => Pattern::Var(ty.push_id_with_min(min, amount)),
            Pattern::Atom(id, gs) => Pattern::Atom(id.push_id_with_min(min, amount), gs.push_id_with_min(min, amount)),
            Pattern::Comp(id, gs, p) => Pattern::Comp(id.push_id_with_min(min, amount), gs.push_id_with_min(min, amount), p.push_id_with_min(min, amount)),
            Pattern::Tuple(v) => Pattern::Tuple(v.push_id_with_min(min, amount)),
        }
    }

    fn pop_id_with_min(&self, min: usize, amount: usize) -> Option<Self> {
        Some(match self {
            Pattern::Var(ty) => Pattern::Var(ty.pop_id_with_min(min, amount)?),
            Pattern::Atom(id, gs) => Pattern::Atom(id.pop_id_with_min(min, amount)?, gs.pop_id_with_min(min, amount)?),
            Pattern::Comp(id, gs, p) => Pattern::Comp(id.pop_id_with_min(min, amount)?, gs.pop_id_with_min(min, amount)?, p.pop_id_with_min(min, amount)?),
            Pattern::Tuple(v) => Pattern::Tuple(v.pop_id_with_min(min, amount)?),
        })
    }
}

impl<T, U: SetLocal<T>> SetLocal<T> for Patterned<U> where Pattern: SetLocal<T> {
    fn set_with_min(&self, min: usize, par: &[T]) -> Self {
        let Patterned(p, e) = self;
        Patterned(p.set_with_min(min + 1, par), e.set_with_min(min + 1, par))
    }
}

impl SetLocal<Exp> for Pattern {
    fn set_with_min(&self, min: usize, par: &[Exp]) -> Self {
        match self {
            Pattern::Var(ty) => Pattern::Var(ty.pop_id_with_min(min, 1).unwrap()),
            Pattern::Atom(id, gs) => Pattern::Atom(id.pop_id_with_min(min, 1).unwrap(), gs.pop_id_with_min(min, 1).unwrap()),
            Pattern::Comp(id, gs, p) => Pattern::Comp(id.pop_id_with_min(min, 1).unwrap(), gs.pop_id_with_min(min, 1).unwrap(), p.set_with_min(min, par)),
            Pattern::Tuple(v) => Pattern::Tuple(v.set_with_min(min, par)),
        }
    }
}

impl SetLocal<Type> for Pattern {
    fn set_with_min(&self, min: usize, par: &[Type]) -> Self {
        match self {
            Pattern::Var(ty) => Pattern::Var(ty.set_with_min(min, par)),
            Pattern::Atom(id, gs) => Pattern::Atom(id.pop_id_with_min(min, 1).unwrap(), gs.set_with_min(min, par)),
            Pattern::Comp(id, gs, p) => Pattern::Comp(id.pop_id_with_min(min, 1).unwrap(), gs.set_with_min(min, par), p.set_with_min(min, par)),
            Pattern::Tuple(v) => Pattern::Tuple(v.set_with_min(min, par)),
        }
    }
}

impl Pattern {
    pub fn to_exp(&self, i: usize) -> Exp {
        match self {
            Pattern::Var(_) => Exp::Var(ID::new(i), vec![]),
            Pattern::Atom(id, gs) => Exp::Var((*id).into(), gs.clone()),
            Pattern::Comp(id, gs, p) => Exp::Call(Box::new(Exp::Var(*id, gs.clone())), Box::new(p.to_exp(i))),
            Pattern::Tuple(v) => {
                let mut i = i;
                Exp::Tuple(v.into_iter().map(|p|{let e = p.to_exp(i); i += p.bounds(); e}).collect())
            }
        }
    }

    pub fn bound(&self) -> Vec<ExpVal> {
        match self {
            Pattern::Var(ty) => vec!(ExpVal::new_empty(ty.push_id(1), 0)),
            Pattern::Atom(_, _) => vec!(),
            Pattern::Comp(_, _, p) => p.bound(),
            Pattern::Tuple(ps) => ps.into_iter().flat_map(|p|p.bound()).collect(),
        }
    }

    pub fn bounds(&self) -> usize {
        match self {
            Pattern::Var(_) => 1,
            Pattern::Atom(_, _) => 0,
            Pattern::Comp(_, _, p) => p.bounds(),
            Pattern::Tuple(ps) => ps.into_iter().map(|p|p.bounds()).sum(),
        }
    }

    pub fn match_exp(&self, e: Exp, env: &Envs) -> Result<Vec<Exp>, ErrID> {
        match &self {
            Pattern::Var(ty) => {
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