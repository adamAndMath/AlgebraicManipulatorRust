use std::marker::PhantomData;
use super::{ ID, PushLocal };

#[derive(Debug)]
pub enum LocalID<T: ?Sized> {
    Global(ID<T>),
    Local(usize, PhantomData<T>),
}

impl<T: ?Sized> LocalID<T> {
    pub fn new(id: usize) -> Self {
        LocalID::Local(id, PhantomData)
    }

    pub fn global(self) -> Result<ID<T>, Self> {
        match self {
            LocalID::Global(id) => Ok(id),
            id => Err(id),
        }
    }
}

impl<T: ?Sized> PushLocal<T> for LocalID<T> {
    fn push_local_with_min(&self, _: PhantomData<T>, min: usize, amount: usize) -> Self {
        match self {
            LocalID::Global(id) => LocalID::Global(*id),
            LocalID::Local(id, p) => LocalID::Local(if *id >= min { id + amount } else { *id }, *p),
        }
    }

    fn push_local(&self, _: PhantomData<T>, amount: usize) -> Self {
        match self {
            LocalID::Global(id) => LocalID::Global(*id),
            LocalID::Local(id, p) => LocalID::Local(id + amount, *p),
        }
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
            LocalID::Local(id, p) => LocalID::Local(*id, *p),
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

impl<T: ?Sized> PartialEq<ID<T>> for LocalID<T> {
    fn eq(&self, rhs: &ID<T>) -> bool {
        match self {
            LocalID::Global(lhs) => lhs == rhs,
            LocalID::Local(_, _) => false,
        }
    }
}

impl<T: ?Sized> Eq for LocalID<T> {}

impl<T: ?Sized> Copy for LocalID<T> {}
