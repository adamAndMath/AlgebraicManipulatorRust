use predef::*;
use envs::TypeVal;
use variance::Variance;
use env::LocalID;
use super::ErrID;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Gen(LocalID<TypeVal>, Vec<(Variance, Type)>),
    Tuple(Vec<Type>),
}

impl Type {
    pub fn call_output(self, arg: &Self) -> Result<Self, ErrID> {
        let (p, b) = get_fn_types(self)?;

        if *arg == p {
            Ok(b)
        } else {
            Err(ErrID::TypeMismatch(arg.clone(), p))
        }
    }
}