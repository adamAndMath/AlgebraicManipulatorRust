use envs::LocalEnvs;
use id::renamed::{ ExpID };
use super::{ Type, Pattern, ErrAst };

#[derive(Debug)]
pub enum Exp {
    Var(String, Vec<Type>),
    Tuple(Vec<Exp>),
    Lambda(Pattern, Box<Exp>),
    Call(Box<Exp>, Box<Exp>),
    Match(Box<Exp>, Vec<(Pattern, Exp)>),
}

impl Exp {
    pub fn to_id(&self, env: &LocalEnvs) -> Result<ExpID, ErrAst> {
        Ok(match self {
            Exp::Var(x, gs) => ExpID::Var(env.exp.get_id(x).map_err(ErrAst::UnknownVar)?, gs.into_iter().map(|g|g.to_id(env)).collect::<Result<_,_>>()?),
            Exp::Tuple(v) => ExpID::Tuple(v.into_iter().map(|e|e.to_id(env)).collect::<Result<_,_>>()?),
            Exp::Lambda(p, e) => {
                let ns = p.bound();
                let p = p.to_id(env)?;
                let ps = ns.into_iter().zip(p.bound()).collect();
                ExpID::Lambda(p, Box::new(e.to_id(&env.scope(ps))?))
            },
            Exp::Call(f, e) => ExpID::Call(Box::new(f.to_id(env)?), Box::new(e.to_id(env)?)),
            Exp::Match(e, ps) => {
                let e_id = e.to_id(env)?;

                ExpID::Match(Box::new(e_id), ps.into_iter().map(|(p,e)|{
                    let ns = p.bound();
                    let p = p.to_id(env)?;
                    let ps = ns.into_iter().zip(p.bound()).collect();
                    Ok((p, e.to_id(&env.scope(ps))?))
                }).collect::<Result<_,ErrAst>>()?)
            },
        })
    }
}
