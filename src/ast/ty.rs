use envs::LocalEnvs;
use id::renamed::TypeID;

#[derive(Debug, Clone)]
pub enum Type {
    Gen(String, Vec<Type>),
    Tuple(Vec<Type>),
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
