use env::Path;
use envs::LocalNamespaces;
use super::{ Type, ErrAst, ToID };
use id::renamed::PatternID;

#[derive(Debug)]
pub enum Pattern {
    Var(String, Type),
    Atom(Path, Vec<Type>),
    Comp(Path, Vec<Type>, Box<Pattern>),
    Tuple(Vec<Pattern>),
}

impl ToID for Pattern {
    type To = PatternID;
    fn to_id(&self, env: &LocalNamespaces) -> Result<PatternID, ErrAst> {
        Ok(match self {
            Pattern::Var(_, ty) => PatternID::Var(ty.to_id(env)?),
            Pattern::Atom(n, gs) => PatternID::Atom(env.get_exp(n)?.global()?, gs.to_id(env)?),
            Pattern::Comp(f, gs, p) => PatternID::Comp(env.get_exp(f)?.global()?, gs.to_id(env)?, p.to_id(env)?),
            Pattern::Tuple(v) => PatternID::Tuple(v.to_id(env)?),
        })
    }
}

impl<'a, T: ToID> ToID for (&'a Pattern, &'a T) {
    type To = (PatternID, T::To);
    fn to_id(&self, env: &LocalNamespaces) -> Result<(PatternID, T::To), ErrAst> {
        let (p, e) = self;
        let ns = p.bound();
        Ok((p.to_id(env)?, e.to_id(&env.scope_exp(ns))?))
    }
}

impl<T: ToID> ToID for (Pattern, T) {
    type To = (PatternID, T::To);
    fn to_id(&self, env: &LocalNamespaces) -> Result<(PatternID, T::To), ErrAst> {
        let (p, e) = (&self.0, &self.1);
        let ns = p.bound();
        Ok((p.to_id(env)?, e.to_id(&env.scope_exp(ns))?))
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
