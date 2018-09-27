use envs::LocalEnvs;
use id::renamed::{ TypeID, ErrID };
use super::ErrAst;

#[derive(Debug, Clone)]
pub enum Type {
    Gen(String, Vec<Type>),
    Tuple(Vec<Type>),
}

impl Type {
    pub fn to_id(&self, env: &LocalEnvs) -> Result<TypeID, ErrAst> {
        Ok(match self {
            Type::Gen(t, gs) => {
                let id = env.ty.get_id(t).map_err(ErrAst::UnknownType)?;
                let ty = env.ty.get(id)?;
                if ty.gen().len() != gs.len() {
                    return Err(ErrAst::ErrID(ErrID::GenericAmount(id, ty.gen().clone())))
                }
                TypeID::Gen(id, ty.gen().into_iter().zip(gs).map(|(v,t)|Ok((*v,t.to_id(env)?))).collect::<Result<_,ErrAst>>()?)
            },
            Type::Tuple(v) => TypeID::Tuple(v.into_iter().map(|t|t.to_id(env)).collect::<Result<_,_>>()?),
        })
    }
}
