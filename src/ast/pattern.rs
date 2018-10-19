use predef::*;
use env::Path;
use envs::LocalEnvs;
use super::{ Type, ErrAst, ToID };
use id::renamed::{ TypeID, PatternID, ErrID };

#[derive(Debug)]
pub enum Pattern<S> {
    Var(S, Type<S>),
    Atom(Path<S>, Vec<Type<S>>),
    Comp(Path<S>, Vec<Type<S>>, Box<Pattern<S>>),
    Tuple(Vec<Pattern<S>>),
}

impl<S: Clone + AsRef<str>> ToID<S> for Pattern<S> {
    type To = PatternID;
    fn to_id<'a>(&self, env: &LocalEnvs<'a>) -> Result<PatternID, ErrAst<S>> {
        Ok(match self {
            Pattern::Var(n, ty) => PatternID::Var(n.as_ref().to_owned(), ty.to_id(env)?),
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

impl<S: Clone + AsRef<str>, T: ToID<S>> ToID<S> for (Pattern<S>, T) {
    type To = (PatternID, T::To);
    fn to_id<'a>(&self, env: &LocalEnvs<'a>) -> Result<(PatternID, T::To), ErrAst<S>> {
        let (p, e) = self;
        let p = p.to_id(env)?;
        let e = e.to_id(&env.scope(p.bound()))?;
        Ok((p, e))
    }
}

impl<S: Clone> Pattern<S> {
    pub fn bound(&self) -> Vec<S> {
        match self {
            Pattern::Var(n, _) => vec![n.clone()],
            Pattern::Atom(_, _) => vec![],
            Pattern::Comp(_, _, p) => p.bound(),
            Pattern::Tuple(v) => v.into_iter().flat_map(|p|p.bound()).collect()
        }
    }
}
