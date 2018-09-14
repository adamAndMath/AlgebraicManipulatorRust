use predef::*;
use env::{ id::ID, local_id::LocalID };
use envs::*;
use variance::Variance::*;
use ty::TypeID;

#[derive(Debug, PartialEq, Clone)]
pub enum PatternID {
    Var(TypeID),
    Atom(ID<ExpVal>),
    Comp(ID<ExpVal>, Box<PatternID>),
    Tuple(Vec<PatternID>),
}

impl PatternID {
    pub fn bound(&self) -> Vec<ExpVal> {
        match self {
            PatternID::Var(ty) => vec!(ExpVal::new_empty(ty.clone())),
            PatternID::Atom(_) => vec!(),
            PatternID::Comp(_, p) => p.bound(),
            PatternID::Tuple(ps) => ps.into_iter().flat_map(|p|p.bound()).collect(),
        }
    }
}