use env::Path;
use envs::LocalEnvs;
use id::renamed::{ ExpID };
use super::{ Type, Pattern, ErrAst, ToID };

#[derive(Debug)]
pub enum Exp<T> {
    Var(Path<T>, Vec<Type<T>>),
    Tuple(Vec<Exp<T>>),
    Closure(Vec<(Pattern<T>, Exp<T>)>),
    Call(Box<Exp<T>>, Box<Exp<T>>),
}

impl<T: Clone + AsRef<str>> ToID<T> for Exp<T> {
    type To = ExpID;
    fn to_id<'a>(&self, env: &LocalEnvs<'a>) -> Result<ExpID, ErrAst<T>> {
        Ok(match self {
            Exp::Var(x, gs) => ExpID::Var(env.exp.get_id(x).map_err(ErrAst::UnknownVar)?, gs.to_id(env)?),
            Exp::Tuple(v) => ExpID::Tuple(v.to_id(env)?),
            Exp::Closure(v) => ExpID::Closure(v.to_id(env)?),
            Exp::Call(f, e) => ExpID::Call(f.to_id(env)?, e.to_id(env)?),
        })
    }
}
