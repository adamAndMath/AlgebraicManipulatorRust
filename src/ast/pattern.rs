use predef::*;
use envs::LocalEnvs;
use super::{ Type, ErrAst };
use id::renamed::{ TypeID, PatternID, ErrID };

#[derive(Debug)]
pub enum Pattern {
    Var(String, Type),
    Atom(String),
    Comp(String, Box<Pattern>),
    Tuple(Vec<Pattern>),
}

impl Pattern {
    pub fn to_id(&self, env: &LocalEnvs) -> Result<PatternID, ErrAst> {
        Ok(match self {
            Pattern::Var(_, ty) => PatternID::Var(ty.to_id(env)?),
            Pattern::Atom(n) => {
                let id = env.exp.get_id(n).map_err(ErrAst::UnknownVar)?;
                let ty = env.exp.get(id)?.ty();
                let ty_def = match ty {
                    TypeID::Gen(ty_id, _) => env.ty.get(ty_id)?,
                    ty => return Err(ErrAst::ErrID(ErrID::NotAtomic(id, ty))),
                };
                let id = id.global()?;
                if !ty_def.contains_atom(&id) { return Err(ErrAst::ErrID(ErrID::NotAtomic(id.into(), ty))) }
                PatternID::Atom(id)
            },
            Pattern::Comp(f, p) => {
                let id = env.exp.get_id(f).map_err(ErrAst::UnknownVar)?;
                let f = env.exp.get(id)?;
                let (_, out_id) = get_fn_types(f.ty())?;
                let ty_out = match out_id {
                    TypeID::Gen(out_id, _) => env.ty.get(out_id),
                    _ => return Err(ErrAst::ErrID(ErrID::NotAtomic(id, out_id))),
                }?;
                let id = id.global()?;
                if !ty_out.contains_comp(&id) { return Err(ErrAst::ErrID(ErrID::NotAtomic(id.into(), out_id))) }
                PatternID::Comp(id, Box::new(p.to_id(env)?))
            },
            Pattern::Tuple(v) => PatternID::Tuple(v.into_iter().map(|p|p.to_id(env)).collect::<Result<_,_>>()?),
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
