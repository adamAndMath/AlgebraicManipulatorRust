use envs::*;
use super::{ Type, Patterned, Exp, Proof, ErrAst, ToID, ToIDMut };
use variance::Variance;
use id::renamed::ElementID;

#[derive(Debug)]
pub enum Element<T> {
    Struct(T, Vec<(Variance, T)>, Option<Type<T>>),
    Enum(T, Vec<(Variance, T)>, Vec<(T, Option<Type<T>>)>),
    Let(T, Vec<T>, Option<Type<T>>, Exp<T>),
    Func(T, Vec<T>, Option<Type<T>>, Vec<Patterned<T, Exp<T>>>),
    Proof(T, Vec<T>, Proof<T>),
}

impl<T: Clone + AsRef<str>> ToIDMut<T> for Element<T> {
    type To = ElementID;
    fn to_id_mut(&self, env: &mut Namespaces) -> Result<ElementID, ErrAst<T>> {
        Ok(match self {
            Element::Struct(n, gs, p) => {
                let var = gs.into_iter().map(|(v,_)|*v).collect();
                let mut names = NameData::new();
                let p = p.to_id(&env.scope_type(&mut names, gs.into_iter().map(|(_,g)|g.to_owned())))?;
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
                    let mut names = NameData::new();
                    let env = &env.scope_type(&mut names, gs.into_iter().map(|(_,g)|g));
                    vs.into_iter().map(|(_,p)|p.to_id(env)).collect::<Result<_,_>>()?
                };
                ElementID::Enum(var, vs)
            },
            Element::Let(n, gs, t, e) => {
                let (t, e) = {
                    let mut names = NameData::new();
                    let env = &env.scope_type(&mut names, gs.clone());
                    (t.to_id(env)?, e.to_id(env)?)
                };
                env.exps.add(n);
                ElementID::Let(gs.len(), t, e)
            },
            Element::Func(n, gs, re, ps) => {
                env.exps.add(n);
                let mut names = NameData::new();
                let env = &env.scope_type(&mut names, gs.clone());
                let re = re.to_id(env)?;
                let e = ps.to_id(env)?;
                ElementID::Func(gs.len(), re, e)
            },
            Element::Proof(n, gs, proof) => {
                let proof = {
                    let mut names = NameData::new();
                    let env = &env.scope_type(&mut names, gs.clone());
                    proof.to_id(env)?
                };
                env.truths.add(n);
                ElementID::Proof(gs.len(), proof)
            }
        })
    }
}