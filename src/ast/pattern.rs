use env::Path;
use envs::{ NameData, Namespaces };
use super::{ Type, ErrAst, ToID, ToIDMut };
use id::renamed::{ PatternID, PatternedID };

#[derive(Debug)]
pub struct Patterned<S, T>(pub Pattern<S>, pub T);

#[derive(Debug)]
pub enum Pattern<S> {
    Var(S, Type<S>),
    Atom(Path<S>, Vec<Type<S>>),
    Comp(Path<S>, Vec<Type<S>>, Box<Pattern<S>>),
    Tuple(Vec<Pattern<S>>),
}

impl<S: Clone + AsRef<str>> ToIDMut<S> for Pattern<S> {
    type To = PatternID;
    fn to_id_mut(&self, env: &mut Namespaces) -> Result<PatternID, ErrAst<S>> {
        Ok(match self {
            Pattern::Var(n, ty) => {
                env.exps.add(n);
                PatternID::Var(ty.to_id(env)?)
            },
            Pattern::Atom(n, gs) => PatternID::Atom(env.get_exp(n)?, gs.to_id(env)?),
            Pattern::Comp(f, gs, p) => PatternID::Comp(env.get_exp(f)?, gs.to_id(env)?, p.to_id_mut(env)?),
            Pattern::Tuple(v) => PatternID::Tuple(v.to_id_mut(env)?),
        })
    }
}

impl<S: Clone + AsRef<str>, T: ToID<S>> ToID<S> for Patterned<S, T> {
    type To = PatternedID<T::To>;
    fn to_id(&self, env: &Namespaces) -> Result<PatternedID<T::To>, ErrAst<S>> {
        let Patterned(p, e) = &self;
        let mut names = NameData::new();
        let mut env = env.scope_empty(&mut names);
        let p = p.to_id_mut(&mut env)?;
        let e = e.to_id(&env)?;
        Ok(PatternedID(p, e))
    }
}
