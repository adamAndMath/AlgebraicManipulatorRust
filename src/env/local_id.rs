use std::marker::PhantomData;
use super::id::ID;

#[derive(Debug)]
pub enum LocalID<T: ?Sized> {
    Global(ID<T>),
    Local(usize, PhantomData<T>),
}

impl<T: ?Sized> LocalID<T> {
    pub fn new(id: usize) -> Self {
        LocalID::Local(id, PhantomData)
    }
}

impl<T: ?Sized> From<ID<T>> for LocalID<T> {
    fn from(id: ID<T>) -> Self {
        LocalID::Global(id)
    }
}

impl<T: ?Sized> Clone for LocalID<T> {
    fn clone(&self) -> Self {
        match self {
            LocalID::Global(id) => LocalID::Global(*id),
            LocalID::Local(id, _) => LocalID::Local(*id, PhantomData),
        }
    }
}

impl<T: ?Sized> PartialEq for LocalID<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LocalID::Global(lhs), LocalID::Global(rhs)) => lhs == rhs,
            (LocalID::Local(lhs, _), LocalID::Local(rhs, _)) => lhs == rhs,
            _ => false,
        }
    }
}

impl<T: ?Sized> Eq for LocalID<T> {}

impl<T: ?Sized> Copy for LocalID<T> {}
