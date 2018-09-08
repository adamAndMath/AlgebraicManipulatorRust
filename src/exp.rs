use exp_id::ExpID;
use envs::{ ExpVal, LocalEnvs };
use ty::Type;

pub enum Exp {
    Var(String),
    Tuple(Vec<Exp>),
    Lambda(Vec<(String, Type)>, Box<Exp>),
    Call(Box<Exp>, Box<Exp>),
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
            Exp::Call(f, e) => ExpID::Call(Box::new(f.to_id(env)?), Box::new(e.to_id(env)?))
        })
    }
}
