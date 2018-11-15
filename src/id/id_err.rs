use env::ID;
use envs::{ ExpVal, TypeVal, TruthVal };
use super::{ Type, Exp, RefType };
use tree::Tree;

#[derive(Debug, PartialEq, Eq)]
pub enum ErrID {
    UnknownExpID(ID<ExpVal>),
    UnknownTypeID(ID<TypeVal>),
    UnknownTruthID(ID<TruthVal>),
    ExpMismatch(Exp, Exp),
    TypeMismatch(Type, Type),
    GenericAmount(usize, usize),
    NotAtomic(ID<ExpVal>, Type),
    IlegalPath(Tree),
    ArgumentAmount(RefType, usize),
    VarNotSet(ID<ExpVal>),
    NoMatch(Exp),
    NotContained,
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
    ID<ExpVal> = UnknownExpID,
    ID<TypeVal> = UnknownTypeID,
    ID<TruthVal> = UnknownTruthID,
    Tree = IlegalPath
}
