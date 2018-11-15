use predef::*;
use envs::TypeVal;
use variance::Variance;
use env::{ ID, PushID };
use super::{ ErrID, SetLocal };

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Gen(ID<TypeVal>, Vec<Type>),
    Tuple(Vec<Type>),
}

impl PushID for (Variance, Type) {
    fn push_id_with_min(&self, min: usize, amount: usize) -> Self {
        (self.0, self.1.push_id_with_min(min, amount))
    }

    fn pop_id_with_min(&self, min: usize, amount: usize) -> Option<Self> {
        Some((self.0, self.1.pop_id_with_min(min, amount)?))
    }
}

impl PushID for Type {
    fn push_id_with_min(&self, min: usize, amount: usize) -> Self {
        match self {
            Type::Gen(id, v) => Type::Gen(id.push_id_with_min(min, amount), v.push_id_with_min(min, amount)),
            Type::Tuple(v) => Type::Tuple(v.push_id_with_min(min, amount)),
        }
    }

    fn pop_id_with_min(&self, min: usize, amount: usize) -> Option<Self> {
        Some(match self {
            Type::Gen(id, v) => Type::Gen(id.pop_id_with_min(min, amount)?, v.pop_id_with_min(min, amount)?),
            Type::Tuple(v) => Type::Tuple(v.pop_id_with_min(min, amount)?),
        })
    }
}

impl SetLocal for Type {
    fn set_with_min(&self, min: usize, par: &[Self]) -> Self {
        match self {
            Type::Gen(id, v) =>
                match id.set(min) {
                    Ok(id) => par[id].push_id(min),
                    Err(id) => Type::Gen(id, v.set_with_min(min, par))
                },
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