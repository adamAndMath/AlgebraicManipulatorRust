use envs::Envs;
use id::{ Type, ErrID };

pub trait TypeCheck {
    fn type_check(&self, env: &Envs) -> Result<Type, ErrID>;
}

impl<T: TypeCheck> TypeCheck for Box<T> {
    fn type_check(&self, env: &Envs) -> Result<Type, ErrID> {
        (&**self).type_check(env)
    }
}

pub trait TypeCheckIter: IntoIterator {
    type Col: IntoIterator<Item = Type>;
    fn type_check(&self, env: &Envs) -> Result<Self::Col, ErrID>;
}

impl<T: TypeCheck> TypeCheckIter for Vec<T> {
    type Col = Vec<Type>;
    fn type_check(&self, env: &Envs) -> Result<Vec<Type>, ErrID> {
        self.iter().map(|e|e.type_check(env)).collect()
    }
}
