use std::collections::HashMap;
use std::marker::PhantomData;
use super::{ LocalID, Namespace, Path, PushLocal };

#[derive(Debug)]
pub enum LocalNamespace<'a, T: 'a> {
    Base(&'a Namespace<'a, T>),
    Scope(&'a LocalNamespace<'a, T>, HashMap<String, usize>),
}

impl<'a, T: 'a> LocalNamespace<'a, T> {
    pub fn new(space: &'a Namespace<'a, T>) -> Self {
        LocalNamespace::Base(space)
    }

    pub fn scope_empty<'b>(&'b self) -> LocalNamespace<'b, T> where 'a: 'b {
        LocalNamespace::Scope(self, HashMap::new())
    }

    pub fn scope<'b, I: IntoIterator>(&'b self, v: I) -> LocalNamespace<'b, T> where 'a: 'b, I::Item: AsRef<str> {
        LocalNamespace::Scope(self, v.into_iter().enumerate().map(|(i,p)|(p.as_ref().to_owned().into(),i)).collect())
    }

    pub fn get<S: Clone + AsRef<str>>(&self, p: &Path<S>) -> Result<LocalID<T>, Path<S>> {
        match self {
            LocalNamespace::Base(space) => space.get(p).map(|id|id.into()),
            LocalNamespace::Scope(space, m) =>
                match m.get(p.name().as_ref()).map(|id|LocalID::new(*id)) {
                    Some(id) if p.len() == 1 => Ok(id),
                    _ => space.get(p).map(|id|id.push_local(PhantomData::<T>, m.len())),
                },
        }
    }
}