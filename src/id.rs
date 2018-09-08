use std::marker::PhantomData;

#[derive(Debug)]
pub struct ID<T: ?Sized>(pub usize, PhantomData<T>);

#[derive(Debug)]
pub enum LocalID<T: ?Sized> {
    Global(usize, PhantomData<T>),
    Local(usize, PhantomData<T>),
}

impl<T: ?Sized> ID<T> {
    pub fn new(id: usize) -> Self {
        ID(id, PhantomData)
    }
}

impl<T: ?Sized> LocalID<T> {
    pub fn new(id: usize) -> Self {
        LocalID::Local(id, PhantomData)
    }
}

impl<T: ?Sized> From<ID<T>> for LocalID<T> {
    fn from(id: ID<T>) -> Self {
        LocalID::Global(id.0, id.1)
    }
}

impl<T: ?Sized> Clone for ID<T> {
    fn clone(&self) -> Self {
        ID(self.0, PhantomData)
    }
}

impl<T: ?Sized> Clone for LocalID<T> {
    fn clone(&self) -> Self {
        match self {
            LocalID::Global(id, _) => LocalID::Global(*id, PhantomData),
            LocalID::Local(id, _) => LocalID::Local(*id, PhantomData),
        }
    }
}

impl<T: ?Sized> PartialEq for ID<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: ?Sized> PartialEq for LocalID<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LocalID::Global(lhs, _), LocalID::Global(rhs, _)) => lhs == rhs,
            (LocalID::Local(lhs, _), LocalID::Local(rhs, _)) => lhs == rhs,
            _ => false,
        }
    }
}

impl<T: ?Sized> Eq for ID<T> {}

impl<T: ?Sized> Eq for LocalID<T> {}

impl<T: ?Sized> Copy for ID<T> {}

impl<T: ?Sized> Copy for LocalID<T> {}
