use envs::LocalEnvs;
use id::renamed::{ TypeID, ErrID };
use super::{ ErrAst, ToID };

#[derive(Debug, Clone)]
pub enum Type {
    Gen(String, Vec<Type>),
    Tuple(Vec<Type>),
}

impl ToID for Type {
    type To = TypeID;
    fn to_id(&self, env: &LocalEnvs) -> Result<TypeID, ErrAst> {
        Ok(match self {
            Type::Gen(t, gs) => {
                let id = env.ty.get_id(t).map_err(ErrAst::UnknownType)?;
                let ty = env.ty.get(id)?;
                if ty.gen().len() != gs.len() {
                    return Err(ErrAst::ErrID(ErrID::GenericAmount(id, ty.gen().clone())))
                }
                TypeID::Gen(id, ty.gen().into_iter().cloned().zip(gs.to_id(env)?).collect())
            },
            Type::Tuple(v) => TypeID::Tuple(v.to_id(env)?),
        })
    }
}
