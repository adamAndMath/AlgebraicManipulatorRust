use std::collections::HashMap;
use std::marker::PhantomData;
use super::{ LocalID, Namespace, Path, PushLocal };

#[derive(Debug)]
pub enum LocalNamespace<'a, T: 'a> {
    Base(&'a Namespace<T>),
    Scope(&'a LocalNamespace<'a, T>, HashMap<Path, usize>),
}

impl<'a, T: 'a> LocalNamespace<'a, T> {
    pub fn new(space: &'a Namespace<T>) -> Self {
        LocalNamespace::Base(space)
    }

    pub fn scope_empty<'b>(&'b self) -> LocalNamespace<'b, T> where 'a: 'b {
        LocalNamespace::Scope(self, HashMap::new())
    }

    pub fn scope<'b, I: IntoIterator>(&'b self, v: I) -> LocalNamespace<'b, T> where 'a: 'b, I::Item: Clone + Into<Path> {
        LocalNamespace::Scope(self, v.into_iter().enumerate().map(|(i,p)|(p.into(),i)).collect())
    }

    pub fn get(&self, p: &Path) -> Result<LocalID<T>, Path> {
        match self {
            LocalNamespace::Base(space) => space.get(p).map(|id|id.into()),
            LocalNamespace::Scope(space, m) =>
                match m.get(p).map(|id|LocalID::new(*id)) {
                    Some(id) => Ok(id),
                    None => space.get(p).map(|id|id.push_local(PhantomData::<T>, m.len())),
                },
        }
    }
}