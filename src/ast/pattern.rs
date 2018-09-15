use predef::*;
use env::LocalID;
use envs::{ LocalEnvs, ExpVal };
use super::Type;
use id::renamed::{ TypeID, PatternID };
use variance::Variance::*;

#[derive(Debug)]
pub enum Pattern {
    Var(String, Type),
    Atom(String),
    Comp(String, Box<Pattern>),
    Tuple(Vec<Pattern>),
}

impl Pattern {
    pub fn to_id(&self, env: &LocalEnvs) -> Option<PatternID> {
        Some(match self {
            Pattern::Var(_, ty) => PatternID::Var(ty.to_id(env)?),
            Pattern::Atom(n) => {
                let id = env.exp.get_id(n)?;
                let ty = env.exp.get(id)?.ty();
                let ty = match ty {
                    TypeID::Gen(ty_id, _) => env.ty.get(ty_id),
                    _ => None,
                }?;
                let id = id.global()?;
                if !ty.contains_atom(&id) { return None }
                PatternID::Atom(id)
            },
            Pattern::Comp(f, p) => {
                let id = env.exp.get_id(f)?;
                let f = env.exp.get(id)?;
                let (_, out_id) = get_fn_types(f.ty())?;
                let ty_out = match out_id {
                    TypeID::Gen(out_id, _) => env.ty.get(out_id),
                    _ => None,
                }?;
                let id = id.global()?;
                if !ty_out.contains_comp(&id) { return None }
                PatternID::Comp(id, Box::new(p.to_id(env)?))
            },
            Pattern::Tuple(v) => PatternID::Tuple(v.into_iter().map(|p|p.to_id(env)).collect::<Option<_>>()?),
        })
    }

    pub fn bound(&self) -> Vec<String> {
        match self {
            Pattern::Var(n, _) => vec![n.clone()],
            Pattern::Atom(_) => vec![],
            Pattern::Comp(_, p) => p.bound(),
            Pattern::Tuple(v) => v.into_iter().flat_map(|p|p.bound()).collect()
        }
    }
}
