use predef::*;
use env::ID;
use envs::*;
use super::Type;

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

    pub fn bound(&self) -> Vec<ExpVal> {
        match self {
            Pattern::Var(ty) => vec!(ExpVal::new_empty(ty.clone(), 0)),
            Pattern::Atom(_) => vec!(),
            Pattern::Comp(_, p) => p.bound(),
            Pattern::Tuple(ps) => ps.into_iter().flat_map(|p|p.bound()).collect(),
        }
    }
}