use env::Path;
use envs::Namespaces;
use id::renamed::{ ExpID };
use super::{ Type, Patterned, ErrAst, ToID };

#[derive(Debug)]
pub enum Exp<T> {
    Var(Path<T>, Vec<Type<T>>),
    Tuple(Vec<Exp<T>>),
    Closure(Vec<Patterned<T, Exp<T>>>),
    Call(Box<Exp<T>>, Box<Exp<T>>),
}

impl<T: Clone + AsRef<str>> ToID<T> for Exp<T> {
    type To = ExpID;
    fn to_id(&self, env: &Namespaces) -> Result<ExpID, ErrAst<T>> {
        Ok(match self {
            Exp::Var(x, gs) => ExpID::Var(env.get_exp(x)?, gs.to_id(env)?),
            Exp::Tuple(v) => ExpID::Tuple(v.to_id(env)?),
            Exp::Closure(v) => ExpID::Closure(v.to_id(env)?),
            Exp::Call(f, e) => ExpID::Call(f.to_id(env)?, e.to_id(env)?),
        })
    }
}
