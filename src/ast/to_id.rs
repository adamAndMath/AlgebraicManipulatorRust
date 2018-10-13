use envs::LocalEnvs;
use ast::ErrAst;

pub trait ToID<'f> {
    type To;
    fn to_id<'a>(&self, env: &LocalEnvs<'f, 'a>) -> Result<Self::To, ErrAst<'f>>;
}

impl<'f, T: ToID<'f>> ToID<'f> for Box<T> {
    type To = Box<T::To>;
    fn to_id<'a>(&self, env: &LocalEnvs<'f, 'a>) -> Result<Self::To, ErrAst<'f>> {
        Ok(Box::new((&**self).to_id(env)?))
    }
}

impl<'f, T: ToID<'f>> ToID<'f> for Option<T> {
    type To = Option<T::To>;
    fn to_id<'a>(&self, env: &LocalEnvs<'f, 'a>) -> Result<Self::To, ErrAst<'f>> {
        self.as_ref().map(|e|e.to_id(env)).transpose()
    }
}

impl<'f, T: ToID<'f>> ToID<'f> for Vec<T> {
    type To = Vec<T::To>;
    fn to_id<'a>(&self, env: &LocalEnvs<'f, 'a>) -> Result<Self::To, ErrAst<'f>> {
        self.into_iter().map(|e|e.to_id(env)).collect()
    }
}
