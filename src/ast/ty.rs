use env::Path;
use envs::LocalNamespaces;
use id::renamed::TypeID;
use super::{ ErrAst, ToID };

#[derive(Debug, Clone)]
pub enum Type {
    Gen(Path, Vec<Type>),
    Tuple(Vec<Type>),
}

impl ToID for Type {
    type To = TypeID;
    fn to_id(&self, env: &LocalNamespaces) -> Result<TypeID, ErrAst> {
        Ok(match self {
            Type::Gen(t, gs) => TypeID::Gen(env.get_type(t)?, gs.to_id(env)?),
            Type::Tuple(v) => TypeID::Tuple(v.to_id(env)?),
        })
    }
}
