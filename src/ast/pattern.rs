use env::Path;
use envs::LocalNamespaces;
use super::{ Type, ErrAst, ToID };
use id::renamed::PatternID;

#[derive(Debug)]
pub enum Pattern<S> {
    Var(S, Type<S>),
    Atom(Path<S>, Vec<Type<S>>),
    Comp(Path<S>, Vec<Type<S>>, Box<Pattern<S>>),
    Tuple(Vec<Pattern<S>>),
}

impl<S: Clone + AsRef<str>> ToID<S> for Pattern<S> {
    type To = PatternID;
    fn to_id(&self, env: &LocalNamespaces) -> Result<PatternID, ErrAst<S>> {
        Ok(match self {
            Pattern::Var(_, ty) => PatternID::Var(ty.to_id(env)?),
            Pattern::Atom(n, gs) => PatternID::Atom(env.get_exp(n)?.global()?, gs.to_id(env)?),
            Pattern::Comp(f, gs, p) => PatternID::Comp(env.get_exp(f)?.global()?, gs.to_id(env)?, p.to_id(env)?),
            Pattern::Tuple(v) => PatternID::Tuple(v.to_id(env)?),
        })
    }
}

impl<'a, S: Clone + AsRef<str>, T: ToID<S>> ToID<S> for (&'a Pattern<S>, &'a T) {
    type To = (PatternID, T::To);
    fn to_id(&self, env: &LocalNamespaces) -> Result<(PatternID, T::To), ErrAst<S>> {
        let (p, e) = self;
        let ns = p.bound();
        Ok((p.to_id(env)?, e.to_id(&env.scope_exp(ns))?))
    }
}

impl<S: Clone + AsRef<str>, T: ToID<S>> ToID<S> for (Pattern<S>, T) {
    type To = (PatternID, T::To);
    fn to_id(&self, env: &LocalNamespaces) -> Result<(PatternID, T::To), ErrAst<S>> {
        let (p, e) = (&self.0, &self.1);
        let ns = p.bound();
        Ok((p.to_id(env)?, e.to_id(&env.scope_exp(ns))?))
    }
}

impl<S> Pattern<S> {
    pub fn bound(&self) -> Vec<&S> {
        match self {
            Pattern::Var(n, _) => vec![n],
            Pattern::Atom(_, _) => vec![],
            Pattern::Comp(_, _, p) => p.bound(),
            Pattern::Tuple(v) => v.into_iter().flat_map(|p|p.bound()).collect()
        }
    }
}
