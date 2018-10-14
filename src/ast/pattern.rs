use predef::*;
use env::Path;
use envs::LocalEnvs;
use super::{ Type, ErrAst, ToID };
use id::renamed::{ TypeID, PatternID, ErrID };

#[derive(Debug)]
pub enum Pattern<'f> {
    Var(&'f str, Type<'f>),
    Atom(Path<'f>, Vec<Type<'f>>),
    Comp(Path<'f>, Vec<Type<'f>>, Box<Pattern<'f>>),
    Tuple(Vec<Pattern<'f>>),
}

impl<'f> ToID<'f> for Pattern<'f> {
    type To = PatternID;
    fn to_id<'a>(&self, env: &LocalEnvs<'a>) -> Result<PatternID, ErrAst<'f>> {
        Ok(match self {
            Pattern::Var(_, ty) => PatternID::Var(ty.to_id(env)?),
            Pattern::Atom(n, gs) => {
                let id = env.exp.get_id(n).map_err(ErrAst::UnknownVar)?;
                let gs = gs.to_id(env)?;
                let ty = env.exp.get(id).ty(&gs);
                let ty_def = match ty {
                    TypeID::Gen(ty_id, _) => env.ty.get(ty_id),
                    ty => return Err(ErrAst::ErrID(ErrID::NotAtomic(id, ty))),
                };
                let id = id.global()?;
                if !ty_def.contains_atom(&id) { return Err(ErrAst::ErrID(ErrID::NotAtomic(id.into(), ty))) }
                PatternID::Atom(id, gs)
            },
            Pattern::Comp(f, gs, p) => {
                let id = env.exp.get_id(f).map_err(ErrAst::UnknownVar)?;
                let gs = gs.to_id(env)?;
                let f = env.exp.get(id);
                let (_, out_id) = get_fn_types(f.ty(&gs))?;
                let ty_out = match out_id {
                    TypeID::Gen(out_id, _) => env.ty.get(out_id),
                    _ => return Err(ErrAst::ErrID(ErrID::NotAtomic(id, out_id))),
                };
                let id = id.global()?;
                if !ty_out.contains_comp(&id) { return Err(ErrAst::ErrID(ErrID::NotAtomic(id.into(), out_id))) }
                PatternID::Comp(id, gs, p.to_id(env)?)
            },
            Pattern::Tuple(v) => PatternID::Tuple(v.to_id(env)?),
        })
    }
}

impl<'f, T: ToID<'f>> ToID<'f> for (Pattern<'f>, T) {
    type To = (PatternID, T::To);
    fn to_id<'a>(&self, env: &LocalEnvs<'a>) -> Result<(PatternID, T::To), ErrAst<'f>> {
        let (p, e) = self;
        let ns = p.bound();
        let p = p.to_id(env)?;
        let ps = ns.into_iter().zip(p.bound()).collect();
        Ok((p, e.to_id(&env.scope(ps))?))
    }
}

impl<'f> Pattern<'f> {
    pub fn bound(&self) -> Vec<&'f str> {
        match self {
            Pattern::Var(n, _) => vec![n],
            Pattern::Atom(_, _) => vec![],
            Pattern::Comp(_, _, p) => p.bound(),
            Pattern::Tuple(v) => v.into_iter().flat_map(|p|p.bound()).collect()
        }
    }
}
