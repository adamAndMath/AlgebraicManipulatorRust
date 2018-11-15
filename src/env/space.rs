use std::collections::HashMap;
use super::{ ID, Path, PushID };

#[derive(Debug)]
pub enum SpaceVal<T> {
    Val(ID<T>),
    Space(Space<T>),
}

#[derive(Debug)]
pub struct Space<T> {
    names: HashMap<String, SpaceVal<T>>,
}

impl<T> Clone for SpaceVal<T> {
    fn clone(&self) -> Self {
        match self {
            SpaceVal::Val(id) => SpaceVal::Val(*id),
            SpaceVal::Space(space) => SpaceVal::Space(space.clone()),
        }
    }
}

impl<T> PushID for Space<T> {
    fn push_id_with_min(&self, min: usize, amount: usize) -> Self {
        Space { names: self.names.iter().map(|(k,v)|(k.clone(), v.push_id_with_min(min, amount))).collect() }
    }

    fn pop_id_with_min(&self, min: usize, amount: usize) -> Option<Self> {
        Some(Space { names: self.names.iter().map(|(k,v)|Some((k.clone(), v.pop_id_with_min(min, amount)?))).collect::<Option<_>>()? })
    }
}

impl<T> PushID for SpaceVal<T> {
    fn push_id_with_min(&self, min: usize, amount: usize) -> Self {
        match self {
            SpaceVal::Val(id) => SpaceVal::Val(id.push_id_with_min(min, amount)),
            SpaceVal::Space(space) => SpaceVal::Space(space.push_id_with_min(min, amount)),
        }
    }

    fn pop_id_with_min(&self, min: usize, amount: usize) -> Option<Self> {
        match self {
            SpaceVal::Val(id) => id.pop_id_with_min(min, amount).map(SpaceVal::Val),
            SpaceVal::Space(space) => space.pop_id_with_min(min, amount).map(SpaceVal::Space),
        }
    }
}

impl<T> Clone for Space<T> {
    fn clone(&self) -> Self {
        Space { names: self.names.clone() }
    }
}

impl<T> Default for Space<T> {
    fn default() -> Self {
        Space { names: HashMap::new() }
    }
}

impl<T> Space<T> {
    pub fn new<S: AsRef<str>, I: IntoIterator<Item = (S, ID<T>)>>(predef: I) -> Self {
        Space {
            names: predef.into_iter().map(|(n, id)|(n.as_ref().to_owned(), SpaceVal::Val(id))).collect(),
        }
    }

    pub fn add<S: AsRef<str>>(&mut self, n: &S, val: SpaceVal<T>) {
        self.names.insert(n.as_ref().to_owned(), val);
    }

    pub fn get<S: Clone + AsRef<str>>(&self, p: &[S]) -> Result<&SpaceVal<T>, Path<S>> {
        let s = &p[0];
        let p = &p[1..];
        let v = self.names.get(s.as_ref());
        if p.is_empty() {
            v.ok_or_else(||s.to_owned().into())
        } else {
            if let Some(SpaceVal::Space(space)) = v {
                space.get(p).map_err(|p|p.prepend(s.clone()))
            } else {
                Err(s.to_owned().into())
            }
        }
    }
}
