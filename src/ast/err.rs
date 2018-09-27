use id::ErrID;
use env::LocalID;
use envs::{ ExpVal, TypeVal, TruthVal };
use tree::Tree;

#[derive(Debug, PartialEq, Eq)]
pub enum ErrAst {
    UnknownVar(String),
    UnknownType(String),
    UnknownTruth(String),
    ArgumentCount(String, usize),
    ErrID(ErrID),
}

macro_rules! impl_from {
    ($($ty:ty),*) => {$(
        impl From<$ty> for ErrAst {
            fn from(id: $ty) -> Self {
                ErrAst::ErrID(id.into())
            }
        }
    )*}
}

impl_from! {
    LocalID<ExpVal>,
    LocalID<TypeVal>,
    LocalID<TruthVal>,
    Tree
}

impl From<ErrID> for ErrAst {
    fn from(err: ErrID) -> Self {
        ErrAst::ErrID(err)
    }
}