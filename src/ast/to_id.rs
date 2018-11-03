use envs::LocalNamespaces;
use ast::ErrAst;

pub trait ToID<S> {
    type To;
    fn to_id(&self, env: &LocalNamespaces) -> Result<Self::To, ErrAst<S>>;
}

impl<S, T: ToID<S>> ToID<S> for Box<T> {
    type To = Box<T::To>;
    fn to_id(&self, env: &LocalNamespaces) -> Result<Self::To, ErrAst<S>> {
        Ok(Box::new((&**self).to_id(env)?))
    }
}

impl<S, T: ToID<S>> ToID<S> for Option<T> {
    type To = Option<T::To>;
    fn to_id(&self, env: &LocalNamespaces) -> Result<Self::To, ErrAst<S>> {
        self.as_ref().map(|e|e.to_id(env)).transpose()
    }
}

impl<S, T: ToID<S>> ToID<S> for Vec<T> {
    type To = Vec<T::To>;
    fn to_id(&self, env: &LocalNamespaces) -> Result<Self::To, ErrAst<S>> {
        self.into_iter().map(|e|e.to_id(env)).collect()
    }
}
