use std::marker::PhantomData;
use predef::*;
use envs::TypeVal;
use variance::Variance;
use env::{ LocalID, PushLocal };
use super::{ ErrID, SetLocal };

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Gen(LocalID<TypeVal>, Vec<Type>),
    Tuple(Vec<Type>),
}

impl PushLocal<TypeVal> for (Variance, Type) {
    fn push_local_with_min(&self, p: PhantomData<TypeVal>, min: usize, amount: usize) -> Self {
        (self.0, self.1.push_local_with_min(p, min, amount))
    }
}

impl PushLocal<TypeVal> for Type {
    fn push_local_with_min(&self, p: PhantomData<TypeVal>, min: usize, amount: usize) -> Self {
        match self {
            Type::Gen(id, v) => Type::Gen(id.push_local_with_min(p, min, amount), v.push_local_with_min(p, min, amount)),
            Type::Tuple(v) => Type::Tuple(v.push_local_with_min(p, min, amount)),
        }
    }
}

impl SetLocal for Type {
    fn set_with_min(&self, min: usize, par: &[Self]) -> Self {
        match self {
            Type::Gen(LocalID::Local(id, _), v) => {
                if *id < min {
                    Type::Gen(LocalID::new(*id), v.set_with_min(min, par))
                } else if id - min >= par.len() {
                    Type::Gen(LocalID::new(id - par.len()), v.set_with_min(min, par))
                } else {
                    par[id - min].push_local(PhantomData::<TypeVal>, min)
                }
            },
            Type::Gen(id, v) => Type::Gen(*id, v.set_with_min(min, par)),
            Type::Tuple(v) => Type::Tuple(v.set_with_min(min, par)),
        }
    }
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