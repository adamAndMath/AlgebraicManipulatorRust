use envs::{ TypeVal, LocalEnvs };
use id::ID;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variance {
    Invariant,
    Covariant,
    Contravariant,
}

#[derive(Debug, Clone)]
pub enum Type {
    Gen(String, Vec<Type>),
    Tuple(Vec<Type>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeID {
    Gen(ID<TypeVal>, Vec<(Variance, TypeID)>),
    Tuple(Vec<TypeID>),
}

impl Type {
    pub fn to_id(&self, env: &LocalEnvs) -> Option<TypeID> {
        Some(match self {
            Type::Gen(t, gs) => {
                let id = env.ty.get_id(t)?;
                let ty = env.ty.get(id)?;
                if ty.gen().len() != gs.len() {
                    panic!("Generic parameter mismatch");
                }
                TypeID::Gen(id, ty.gen().into_iter().zip(gs).map(|(v,t)|Some((*v,t.to_id(env)?))).collect::<Option<_>>()?)
            },
            Type::Tuple(v) => TypeID::Tuple(v.into_iter().map(|t|t.to_id(env)).collect::<Option<_>>()?),
        })
    }
}
