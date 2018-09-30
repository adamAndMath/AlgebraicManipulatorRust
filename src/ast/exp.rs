use envs::LocalEnvs;
use id::renamed::{ ExpID };
use super::{ Type, Pattern, ErrAst, ToID };

#[derive(Debug)]
pub enum Exp {
    Var(String, Vec<Type>),
    Tuple(Vec<Exp>),
    Lambda(Pattern, Box<Exp>),
    Call(Box<Exp>, Box<Exp>),
    Match(Box<Exp>, Vec<(Pattern, Exp)>),
}

impl ToID for Exp {
    type To = ExpID;
    fn to_id(&self, env: &LocalEnvs) -> Result<ExpID, ErrAst> {
        Ok(match self {
            Exp::Var(x, gs) => ExpID::Var(env.exp.get_id(x).map_err(ErrAst::UnknownVar)?, gs.to_id(env)?),
            Exp::Tuple(v) => ExpID::Tuple(v.to_id(env)?),
            Exp::Lambda(p, e) => {
                let (p, e) = (p, e).to_id(env)?;
                ExpID::Lambda(p, e)
            },
            Exp::Call(f, e) => ExpID::Call(f.to_id(env)?, e.to_id(env)?),
            Exp::Match(e, ps) => ExpID::Match(e.to_id(env)?, ps.to_id(env)?),
        })
    }
}
