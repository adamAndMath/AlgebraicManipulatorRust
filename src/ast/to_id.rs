use envs::LocalEnvs;
use ast::ErrAst;

pub trait ToID {
    type To;
    fn to_id(&self, env: &LocalEnvs) -> Result<Self::To, ErrAst>;
}

impl<T: ToID> ToID for Box<T> {
    type To = Box<T::To>;
    fn to_id(&self, env: &LocalEnvs) -> Result<Self::To, ErrAst> {
        Ok(Box::new((&**self).to_id(env)?))
    }
}

impl<T: ToID> ToID for Option<T> {
    type To = Option<T::To>;
    fn to_id(&self, env: &LocalEnvs) -> Result<Self::To, ErrAst> {
        self.as_ref().map(|e|e.to_id(env)).transpose()
    }
}

impl<T: ToID> ToID for Vec<T> {
    type To = Vec<T::To>;
    fn to_id(&self, env: &LocalEnvs) -> Result<Self::To, ErrAst> {
        self.into_iter().map(|e|e.to_id(env)).collect()
    }
}
