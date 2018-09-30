use envs::LocalEnvs;
use id::renamed::{ ExpID };
use super::{ Type, Pattern, ErrAst, ToID };

#[derive(Debug)]
pub enum Exp {
    Var(String, Vec<Type>),
    Tuple(Vec<Exp>),
    Closure(Vec<(Pattern, Exp)>),
    Call(Box<Exp>, Box<Exp>),
}

impl ToID for Exp {
    type To = ExpID;
    fn to_id(&self, env: &LocalEnvs) -> Result<ExpID, ErrAst> {
        Ok(match self {
            Exp::Var(x, gs) => ExpID::Var(env.exp.get_id(x).map_err(ErrAst::UnknownVar)?, gs.to_id(env)?),
            Exp::Tuple(v) => ExpID::Tuple(v.to_id(env)?),
            Exp::Closure(v) => ExpID::Closure(v.to_id(env)?),
            Exp::Call(f, e) => ExpID::Call(f.to_id(env)?, e.to_id(env)?),
        })
    }
}
