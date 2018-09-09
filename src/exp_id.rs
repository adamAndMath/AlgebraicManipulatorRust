use id::*;
use ty::{ Variance::*, TypeID };
use pattern::PatternID;
use envs::{ ExpVal, LocalEnvs };

#[derive(Debug, PartialEq, Clone)]
pub enum ExpID {
    Var(LocalID<ExpVal>),
    Tuple(Vec<ExpID>),
    Lambda(Vec<TypeID>, Box<ExpID>),
    Call(Box<ExpID>, Box<ExpID>),
    Match(Box<ExpID>, Vec<(PatternID, ExpID)>),
}

impl ExpID {
    pub fn type_check(&self, env: &LocalEnvs) -> Option<TypeID> {
        Some(match self {
            ExpID::Var(x) => env.exp.get(*x)?.ty(),
            ExpID::Tuple(v) => TypeID::Tuple(v.into_iter().map(|e|e.type_check(env)).collect::<Option<_>>()?),
            ExpID::Lambda(xs, e) => {
                let p = if let [ref x] = xs[..] {
                    x.clone()
                } else {
                    TypeID::Tuple(xs.clone())
                };
                let b = e.type_check(&env.scope_anon(xs.clone().into_iter().map(ExpVal::new_empty).collect()))?;
                TypeID::Gen(ID::new(0), vec![(Contravariant, p), (Covariant, b)])
            },
            ExpID::Call(f, e) => {
                let f = f.type_check(env)?;
                let e = e.type_check(env)?;
                
                if let TypeID::Gen(f_id, v) = f {
                    if f_id != ID::new(0) {
                        panic!("Not a function");
                    }
                    if let [(Contravariant, ref p), (Covariant, ref b)] = v[..] {
                        if p == &e {
                            b.clone()
                        } else {
                            panic!("Type missmatch");
                        }
                    } else {
                        unreachable!()
                    }
                } else {
                    panic!("Not a function");
                }
            },
            ExpID::Match(_, ps) => TypeID::Tuple(ps.into_iter().map(|(p,e)|e.type_check(&env.scope_anon(p.bound()))).collect::<Option<_>>()?),
        })
    }
}
