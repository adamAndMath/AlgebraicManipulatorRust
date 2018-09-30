use predef::*;
use envs::LocalEnvs;
use super::{ Type, ErrAst, ToID };
use id::renamed::{ TypeID, PatternID, ErrID };

#[derive(Debug)]
pub enum Pattern {
    Var(String, Type),
    Atom(String, Vec<Type>),
    Comp(String, Vec<Type>, Box<Pattern>),
    Tuple(Vec<Pattern>),
}

impl ToID for Pattern {
    type To = PatternID;
    fn to_id(&self, env: &LocalEnvs) -> Result<PatternID, ErrAst> {
        Ok(match self {
            Pattern::Var(_, ty) => PatternID::Var(ty.to_id(env)?),
            Pattern::Atom(n, gs) => {
                let id = env.exp.get_id(n).map_err(ErrAst::UnknownVar)?;
                let gs = gs.to_id(env)?;
                let ty = env.exp.get(id)?.ty(&gs);
                let ty_def = match ty {
                    TypeID::Gen(ty_id, _) => env.ty.get(ty_id)?,
                    ty => return Err(ErrAst::ErrID(ErrID::NotAtomic(id, ty))),
                };
                let id = id.global()?;
                if !ty_def.contains_atom(&id) { return Err(ErrAst::ErrID(ErrID::NotAtomic(id.into(), ty))) }
                PatternID::Atom(id, gs)
            },
            Pattern::Comp(f, gs, p) => {
                let id = env.exp.get_id(f).map_err(ErrAst::UnknownVar)?;
                let gs = gs.to_id(env)?;
                let f = env.exp.get(id)?;
                let (_, out_id) = get_fn_types(f.ty(&gs))?;
                let ty_out = match out_id {
                    TypeID::Gen(out_id, _) => env.ty.get(out_id),
                    _ => return Err(ErrAst::ErrID(ErrID::NotAtomic(id, out_id))),
                }?;
                let id = id.global()?;
                if !ty_out.contains_comp(&id) { return Err(ErrAst::ErrID(ErrID::NotAtomic(id.into(), out_id))) }
                PatternID::Comp(id, gs, p.to_id(env)?)
            },
            Pattern::Tuple(v) => PatternID::Tuple(v.to_id(env)?),
        })
    }
}

impl<'a, T: ToID> ToID for (&'a Pattern, &'a T) {
    type To = (PatternID, T::To);
    fn to_id(&self, env: &LocalEnvs) -> Result<(PatternID, T::To), ErrAst> {
        let (p, e) = self;
        let ns = p.bound();
        let p = p.to_id(env)?;
        let ps = ns.into_iter().zip(p.bound()).collect();
        Ok((p, e.to_id(&env.scope(ps))?))
    }
}

impl<T: ToID> ToID for (Pattern, T) {
    type To = (PatternID, T::To);
    fn to_id(&self, env: &LocalEnvs) -> Result<(PatternID, T::To), ErrAst> {
        (&self.0, &self.1).to_id(env)
    }
}

impl Pattern {
    pub fn bound(&self) -> Vec<String> {
        match self {
            Pattern::Var(n, _) => vec![n.clone()],
            Pattern::Atom(_, _) => vec![],
            Pattern::Comp(_, _, p) => p.bound(),
            Pattern::Tuple(v) => v.into_iter().flat_map(|p|p.bound()).collect()
        }
    }
}
