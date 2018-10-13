use id::ErrID;
use env::{ LocalID, Path };
use envs::{ ExpVal, TypeVal, TruthVal };
use parser::Error;
use pest::error::ErrorVariant;
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

impl<'f> Into<Error> for ErrAst<'f> {
    fn into(self) -> Error {
        match self {
            ErrAst::UnknownVar(p) => Error::new_from_span(ErrorVariant::CustomError { message: "Unknown variable".to_owned() }, p.as_span()),
            ErrAst::UnknownType(p) => Error::new_from_span(ErrorVariant::CustomError { message: "Unknown type".to_owned() }, p.as_span()),
            ErrAst::UnknownTruth(p) => Error::new_from_span(ErrorVariant::CustomError { message: "Unknown truth".to_owned() }, p.as_span()),
            ErrAst::UndefinedPath(p) => Error::new_from_span(ErrorVariant::CustomError { message: "Path doesn't exist".to_owned() }, p.as_span()),
            e => panic!("{:?}", e),
        }
    }
}