use env::Path;
use envs::LocalNamespaces;
use id::renamed::TypeID;
use super::{ ErrAst, ToID };

#[derive(Debug, Clone)]
pub enum Type<T> {
    Gen(Path<T>, Vec<Type<T>>),
    Tuple(Vec<Type<T>>),
}

impl<T: Clone + AsRef<str>> ToID<T> for Type<T> {
    type To = TypeID;
    fn to_id(&self, env: &LocalNamespaces) -> Result<TypeID, ErrAst<T>> {
        Ok(match self {
            Type::Gen(t, gs) => TypeID::Gen(env.get_type(t)?, gs.to_id(env)?),
            Type::Tuple(v) => TypeID::Tuple(v.to_id(env)?),
        })
    }
}
