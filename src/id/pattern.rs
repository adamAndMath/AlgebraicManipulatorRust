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
    pub fn bound(&self) -> Vec<ExpVal> {
        match self {
            Pattern::Var(ty) => vec!(ExpVal::new_empty(ty.clone())),
            Pattern::Atom(_) => vec!(),
            Pattern::Comp(_, p) => p.bound(),
            Pattern::Tuple(ps) => ps.into_iter().flat_map(|p|p.bound()).collect(),
        }
    }
}