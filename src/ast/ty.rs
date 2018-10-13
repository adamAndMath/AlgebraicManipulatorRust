use env::Path;
use envs::LocalEnvs;
use id::renamed::{ TypeID, ErrID };
use super::{ ErrAst, ToID };

#[derive(Debug, Clone)]
pub enum Type<'f> {
    Gen(Path<'f>, Vec<Type<'f>>),
    Tuple(Vec<Type<'f>>),
}

impl<'f> ToID<'f> for Type<'f> {
    type To = TypeID;
    fn to_id<'a>(&self, env: &LocalEnvs<'f, 'a>) -> Result<TypeID, ErrAst<'f>> {
        Ok(match self {
            Type::Gen(t, gs) => {
                let id = env.ty.get_id(t).map_err(ErrAst::UnknownType)?;
                let ty = env.ty.get(id)?;
                if ty.gen().len() != gs.len() {
                    return Err(ErrAst::ErrID(ErrID::GenericAmount(gs.len(), ty.gen().len())))
                }
                TypeID::Gen(id, ty.gen().into_iter().cloned().zip(gs.to_id(env)?).collect())
            },
            Type::Tuple(v) => TypeID::Tuple(v.to_id(env)?),
        })
    }
}
