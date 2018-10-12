use std::collections::HashMap;
use super::ID;

#[derive(Debug)]
pub enum Val<T: ?Sized> {
    ID(ID<T>),
    Space(HashMap<String, Val<T>>),
}

impl<T: ?Sized> Clone for Val<T> {
    fn clone(&self) -> Self {
        match self {
            Val::ID(id) => Val::ID(*id),
            Val::Space(m) => Val::Space(m.clone()),
        }
    }
}

impl<T: ?Sized> From<ID<T>> for Val<T> {
    fn from(id: ID<T>) -> Self {
        Val::ID(id)
    }
}