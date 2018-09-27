use predef::*;
use env::{ ID, LocalID };
use envs::*;
use super::{ Type, Exp, ErrID };

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Pattern {
    Var(Type),
    Atom(ID<ExpVal>),
    Comp(ID<ExpVal>, Box<Pattern>),
    Tuple(Vec<Pattern>),
}

impl Pattern {
    pub fn type_check(&self, env: &LocalEnvs) -> Result<Type, ErrID> {
        Ok(match self {
            Pattern::Var(ty) => ty.clone(),
            Pattern::Atom(id) => env.exp.get(*id)?.ty(),
            Pattern::Comp(id, p) => {
                let f = env.exp.get(*id)?;
                let t = p.type_check(env)?;

                let (p, b) = get_fn_types(f.ty())?;
                if t != p { return Err(ErrID::TypeMismatch(t, p)) }
                b
            },
            Pattern::Tuple(v) => Type::Tuple(v.into_iter().map(|p|p.type_check(env)).collect::<Result<_, ErrID>>()?),
        })
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
                    if *f == *c {
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