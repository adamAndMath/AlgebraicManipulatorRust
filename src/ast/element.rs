use envs::*;
use super::{ Type, Pattern, Exp, Proof, ErrAst, ToID };
use variance::Variance;
use id::renamed::{ ElementID };

#[derive(Debug)]
pub enum Element<T> {
    Struct(T, Vec<(Variance, T)>, Option<Type<T>>),
    Enum(T, Vec<(Variance, T)>, Vec<(T, Option<Type<T>>)>),
    Let(T, Vec<T>, Option<Type<T>>, Exp<T>),
    Func(T, Vec<T>, Option<Type<T>>, Vec<(Pattern<T>, Exp<T>)>),
    Proof(T, Vec<T>, Option<Pattern<T>>, Proof<T>),
}

impl<T: Clone + AsRef<str>> Element<T> {
    pub fn to_id(&self, env: &mut Namespaces) -> Result<ElementID, ErrAst<T>> {
        Ok(match self {
            Element::Struct(n, gs, p) => {
                let var = gs.into_iter().map(|(v,_)|*v).collect();
                let p = p.to_id(&env.local().scope_type(gs.into_iter().map(|(_,g)|g.to_owned())))?;
                env.types.add(n);
                env.exps.add(n);
                ElementID::Struct(var, p)
            },
            Element::Enum(n, gs, vs) => {
                env.types.add(n);
                {
                    let mut sub_space = env.exps.sub_space(n);
                    vs.into_iter().for_each(|(v,_)|{sub_space.add(v);});
                }
                let var = gs.into_iter().map(|(v,_)|*v).collect();
                let vs = {
                    let env = env.local();
                    let env = &env.scope_type(gs.into_iter().map(|(_,g)|g));
                    vs.into_iter().map(|(_,p)|p.to_id(env)).collect::<Result<_,_>>()?
                };
                ElementID::Enum(var, vs)
            },
            Element::Let(n, gs, t, e) => {
                let (t, e) = {
                    let env = env.local();
                    let env = &env.scope_type(gs.clone());
                    (t.to_id(env)?, e.to_id(env)?)
                };
                env.exps.add(n);
                ElementID::Let(gs.len(), t, e)
            },
            Element::Func(n, gs, re, ps) => {
                env.exps.add(n);
                let env = env.local();
                let env = &env.scope_type(gs.clone());
                let re = re.to_id(env)?;
                let e = ps.to_id(env)?;
                ElementID::Func(gs.len(), re, e)
            },
            Element::Proof(n, gs, p, proof) => {
                let (p, proof) = {
                    let env = env.local();
                    let env = &env.scope_type(gs.clone());
                    match p {
                        Some(p) => {
                            let (p, proof) = (p, proof).to_id(env)?;
                            (Some(p), proof)
                        },
                        None => (None, proof.to_id(env)?),
                    }
                };
                env.truths.add(n);
                ElementID::Proof(gs.len(), p, proof)
            }
        })
    }
}