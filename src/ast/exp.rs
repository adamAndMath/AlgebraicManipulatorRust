use env::Path;
use envs::LocalEnvs;
use id::renamed::{ ExpID };
use super::{ Type, Pattern, ErrAst, ToID };

#[derive(Debug)]
pub enum Exp<'f> {
    Var(Path<'f>, Vec<Type<'f>>),
    Tuple(Vec<Exp<'f>>),
    Closure(Vec<(Pattern<'f>, Exp<'f>)>),
    Call(Box<Exp<'f>>, Box<Exp<'f>>),
}

impl<'f> ToID<'f> for Exp<'f> {
    type To = ExpID;
    fn to_id<'a>(&self, env: &LocalEnvs<'f, 'a>) -> Result<ExpID, ErrAst<'f>> {
        Ok(match self {
            Exp::Var(x, gs) => ExpID::Var(env.exp.get_id(x).map_err(ErrAst::UnknownVar)?, gs.to_id(env)?),
            Exp::Tuple(v) => ExpID::Tuple(v.to_id(env)?),
            Exp::Closure(v) => ExpID::Closure(v.to_id(env)?),
            Exp::Call(f, e) => ExpID::Call(f.to_id(env)?, e.to_id(env)?),
        })
    }
}
