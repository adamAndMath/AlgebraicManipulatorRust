use envs::Namespaces;
use ast::ErrAst;

pub trait ToIDMut<S> {
    type To;
    fn to_id_mut(&self, env: &mut Namespaces) -> Result<Self::To, ErrAst<S>>;
}

impl<S, T: ToIDMut<S>> ToIDMut<S> for Box<T> {
    type To = Box<T::To>;
    fn to_id_mut(&self, env: &mut Namespaces) -> Result<Self::To, ErrAst<S>> {
        Ok(Box::new((&**self).to_id_mut(env)?))
    }
}

impl<S, T: ToIDMut<S>> ToIDMut<S> for Option<T> {
    type To = Option<T::To>;
    fn to_id_mut(&self, env: &mut Namespaces) -> Result<Self::To, ErrAst<S>> {
        self.as_ref().map(|e|e.to_id_mut(env)).transpose()
    }
}

impl<S, T: ToIDMut<S>> ToIDMut<S> for Vec<T> {
    type To = Vec<T::To>;
    fn to_id_mut(&self, env: &mut Namespaces) -> Result<Self::To, ErrAst<S>> {
        self.into_iter().map(|e|e.to_id_mut(env)).collect()
    }
}
