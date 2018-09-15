use predef::*;
use env::LocalID;
use variance::Variance::*;
use super::{ Type, Pattern };
use envs::{ ExpVal, LocalEnvs };

#[derive(Debug, PartialEq, Clone)]
pub enum Exp {
    Var(LocalID<ExpVal>),
    Tuple(Vec<Exp>),
    Lambda(Pattern, Box<Exp>),
    Call(Box<Exp>, Box<Exp>),
    Match(Box<Exp>, Vec<(Pattern, Exp)>),
}

impl Exp {
    pub fn type_check(&self, env: &LocalEnvs) -> Option<Type> {
        Some(match self {
            Exp::Var(x) => env.exp.get(*x)?.ty(),
            Exp::Tuple(v) => Type::Tuple(v.into_iter().map(|e|e.type_check(env)).collect::<Option<_>>()?),
            Exp::Lambda(p, e) => {
                let b = e.type_check(&env.scope_anon(p.bound()))?;
                Type::Gen(FN_ID.into(), vec![(Contravariant, p.type_check(env)?), (Covariant, b)])
            },
            Exp::Call(f, e) => {
                let f = f.type_check(env)?;
                let e = e.type_check(env)?;
                
                let (p, b) = get_fn_types(f)?;

                if p != e { return None; }
                
                b
            },
            Exp::Match(_, ps) => Type::Tuple(ps.into_iter().map(|(p,e)|e.type_check(&env.scope_anon(p.bound()))).collect::<Option<_>>()?),
        })
    }
}
