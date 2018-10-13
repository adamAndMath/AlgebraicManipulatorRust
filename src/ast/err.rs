use id::ErrID;
use env::{ LocalID, Path };
use envs::{ ExpVal, TypeVal, TruthVal };
use tree::Tree;

#[derive(Debug, PartialEq, Eq)]
pub enum ErrAst<'f> {
    UnknownVar(Path<'f>),
    UnknownType(Path<'f>),
    UnknownTruth(Path<'f>),
    UndefinedPath(Path<'f>),
    ErrID(ErrID),
}

macro_rules! impl_from {
    ($($ty:ty),*) => {$(
        impl<'f> From<$ty> for ErrAst<'f> {
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

impl<'f> From<ErrID> for ErrAst<'f> {
    fn from(err: ErrID) -> Self {
        ErrAst::ErrID(err)
    }
}