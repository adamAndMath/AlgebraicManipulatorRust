use envs::*;
use env::Path;
use super::{ Type, Pattern, Exp, Proof, ErrAst, ToID };
use variance::Variance;
use id::renamed::{ ElementID };

pub enum Element {
    Module(String, Vec<Element>),
    Using(Path),
    Struct(String, Vec<(Variance, String)>, Option<Type>),
    Enum(String, Vec<(Variance, String)>, Vec<(String, Option<Type>)>),
    Let(String, Vec<String>, Option<Type>, Exp),
    Func(String, Vec<String>, Option<Type>, Vec<(Pattern, Exp)>),
    Proof(String, Vec<String>, Option<Pattern>, Proof),
}

impl Element {
    pub fn to_id(&self, env: &mut Namespaces) -> Result<Vec<ElementID>, ErrAst> {
        Ok(vec![match self {
            Element::Module(n, v) => {
                return env.sub_space(n.clone(), |env|v.into_iter().map(|e|e.to_id(env)).collect::<Result<Vec<_>,_>>().map(|v|v.into_iter().flatten().collect()))
            },
            Element::Using(p) => {
                return env.alias(p.name(), p).map(|()|vec![]).map_err(ErrAst::UndefinedPath);
            },
            Element::Struct(n, gs, p) => {
                let var = gs.into_iter().map(|(v,_)|*v).collect();
                let p = p.to_id(&env.local().scope_type(gs.into_iter().map(|(_,g)|g.to_owned())))?;
                env.types.add(n.to_owned());
                env.exps.add(n.to_owned());
                ElementID::Struct(var, p)
            },
            Element::Enum(n, gs, vs) => {
                env.types.add(n.to_owned());
                let mut sub_space = env.exps.sub_space(n.clone());
                vs.into_iter().for_each(|(v,_)|{sub_space.add(v.to_owned());});
                env.exps.add_space(n.to_owned(), sub_space);
                let var = gs.into_iter().map(|(v,_)|*v).collect();
                let vs = {
                    let env = env.local();
                    let env = &env.scope_type(gs.into_iter().map(|(_,g)|g.to_owned()));
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
                env.exps.add(n.to_owned());
                ElementID::Let(gs.len(), t, e)
            },
            Element::Func(n, gs, re, ps) => {
                env.exps.add(n.to_owned());
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
                env.truths.add(n.to_owned());
                ElementID::Proof(gs.len(), p, proof)
            }
        }])
    }
}