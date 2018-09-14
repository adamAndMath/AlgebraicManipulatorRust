use envs::{ ExpVal, LocalEnvs };
use id::renamed::ExpID;
use super::{ Type, Pattern };

pub enum Exp {
    Var(String),
    Tuple(Vec<Exp>),
    Lambda(Vec<(String, Type)>, Box<Exp>),
    Call(Box<Exp>, Box<Exp>),
    Match(Box<Exp>, Vec<(Pattern, Exp)>),
}

impl Exp {
    pub fn to_id(&self, env: &LocalEnvs) -> Option<ExpID> {
        Some(match self {
            Exp::Var(x) => ExpID::Var(env.exp.get_id(x).unwrap()),
            Exp::Tuple(v) => ExpID::Tuple(v.into_iter().map(|e|e.to_id(env)).collect::<Option<_>>()?),
            Exp::Lambda(ps, e) => {
                let ps: Vec<_> = ps.iter().map(|(n,t)|Some((n.clone(),ExpVal::new_empty(t.to_id(env)?)))).collect::<Option<_>>()?;
                let ts: Vec<_> = ps.iter().map(|(_,p)|p.ty()).collect();
                ExpID::Lambda(ts, Box::new(e.to_id(&env.scope(ps))?))
            },
            Exp::Call(f, e) => ExpID::Call(Box::new(f.to_id(env)?), Box::new(e.to_id(env)?)),
            Exp::Match(e, ps) => {
                let e_id = e.to_id(env)?;
                let e_ty = e_id.type_check(env)?;

                ExpID::Match(Box::new(e_id), ps.into_iter().map(|(p,e)|{
                    let (p, v) = p.to_id(&e_ty, env)?;
                    Some((p, e.to_id(&env.scope(v))?))
                }).collect::<Option<_>>()?)
            },
        })
    }
}
