use std::marker::PhantomData;

#[derive(Debug)]
pub enum ID<T: ?Sized> {
    Predef(usize, PhantomData<T>),
    Normal(usize, PhantomData<T>),
}

impl<T: ?Sized> ID<T> {
    pub fn new(id: usize) -> Self {
        ID::Normal(id, PhantomData)
    }
}

impl<T: ?Sized> Clone for ID<T> {
    fn clone(&self) -> Self {
        match self {
            ID::Predef(id, p) => ID::Predef(*id, p.clone()),
            ID::Normal(id, p) => ID::Normal(*id, p.clone()),
        }
    }
}

impl<T: ?Sized> PartialEq for ID<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ID::Predef(lhs, _), ID::Predef(rhs, _)) => lhs == rhs,
            (ID::Normal(lhs, _), ID::Normal(rhs, _)) => lhs == rhs,
            _ => false,
        }
    }
}

impl<T: ?Sized> Eq for ID<T> {}

impl<T: ?Sized> Copy for ID<T> {}
