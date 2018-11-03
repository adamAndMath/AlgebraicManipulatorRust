use id::ErrID;
use env::{ LocalID, Path };
use envs::{ ExpVal, TypeVal, TruthVal };
use parser::Error;
use pest::error::ErrorVariant;
use ast::Word;
use tree::Tree;

#[derive(Debug, PartialEq, Eq)]
pub enum ErrAst<T> {
    UnknownVar(Path<T>),
    UnknownType(Path<T>),
    UnknownTruth(Path<T>),
    UndefinedPath(Path<T>),
    ErrID(ErrID),
}

macro_rules! impl_from {
    ($($ty:ty),*) => {$(
        impl<T> From<$ty> for ErrAst<T> {
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

impl<T> From<ErrID> for ErrAst<T> {
    fn from(err: ErrID) -> Self {
        ErrAst::ErrID(err)
    }
}

impl<'f> Into<Error> for ErrAst<Word<'f>> {
    fn into(self) -> Error {
        match self {
            ErrAst::UnknownVar(p) => Error::new_from_span(ErrorVariant::CustomError { message: "Unknown variable".to_owned() }, p.name().as_span()),
            ErrAst::UnknownType(p) => Error::new_from_span(ErrorVariant::CustomError { message: "Unknown type".to_owned() }, p.name().as_span()),
            ErrAst::UnknownTruth(p) => Error::new_from_span(ErrorVariant::CustomError { message: "Unknown truth".to_owned() }, p.name().as_span()),
            ErrAst::UndefinedPath(p) => Error::new_from_span(ErrorVariant::CustomError { message: format!("The path \"{}\" doesn't exist", p) }, p.name().as_span()),
            e => panic!("{:?}", e),
        }
    }
}