use id::LocalID;
use ty::{ Variance::*, TypeID };
use envs::LocalEnvs;

#[derive(Debug, PartialEq)]
pub enum ExpID {
    Var(LocalID),
    Tuple(Vec<ExpID>),
    Lambda(Vec<TypeID>, Box<ExpID>),
    Call(Box<ExpID>, Box<ExpID>),
}

impl ExpID {
    pub fn type_check(&self, env: &LocalEnvs) -> Option<TypeID> {
        Some(match self {
            ExpID::Var(x) => env.exp.get(*x)?.1.clone(),
            ExpID::Tuple(v) => TypeID::Tuple(v.into_iter().map(|e|e.type_check(env)).collect::<Option<_>>()?),
            ExpID::Lambda(xs, e) => {
                let p = if let [ref x] = xs[..] {
                    x.clone()
                } else {
                    TypeID::Tuple(xs.clone())
                };
                let b = e.type_check(&env.scope_anon(xs.into_iter().map(|x|(None,x.clone())).collect()))?;
                TypeID::Gen(0, vec![(Contravariant, p), (Covariant, b)])
            },
            ExpID::Call(f, e) => {
                let f = f.type_check(env)?;
                let e = e.type_check(env)?;
                
                if let TypeID::Gen(0, v) = f {
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
            }
        })
    }
}
