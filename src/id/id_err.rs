use env::LocalID;
use envs::{ ExpVal, TypeVal, TruthVal };
use variance::Variance;
use super::{ Type, Exp };
use tree::Tree;

#[derive(Debug, PartialEq, Eq)]
pub enum ErrID {
    UnknownExpID(LocalID<ExpVal>),
    UnknownTypeID(LocalID<TypeVal>),
    UnknownTruthID(LocalID<TruthVal>),
    ExpMismatch(Exp, Exp),
    TypeMismatch(Type, Type),
    GenericAmount(LocalID<TypeVal>, Vec<Variance>),
    InvalidArguments(Vec<Exp>),
    NotAtomic(LocalID<ExpVal>, Type),
    IlegalPath(Tree),
    VarNotSet(LocalID<ExpVal>),
    NoMatch(Exp),
}

macro_rules! impl_from {
    ($($ty:ty = $var:ident),*) => {$(
        impl From<$ty> for ErrID {
            fn from(id: $ty) -> Self {
                ErrID::$var(id)
            }
        }
    )*}
}

impl_from!{
    LocalID<ExpVal> = UnknownExpID,
    LocalID<TypeVal> = UnknownTypeID,
    LocalID<TruthVal> = UnknownTruthID,
    Tree = IlegalPath
}
