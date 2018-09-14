use std::marker::PhantomData;

#[derive(Debug)]
pub struct ID<T: ?Sized>(pub usize, pub PhantomData<T>);

impl<T: ?Sized> ID<T> {
    pub fn new(id: usize) -> Self {
        ID(id, PhantomData)
    }
}

impl<T: ?Sized> Clone for ID<T> {
    fn clone(&self) -> Self {
        ID(self.0, PhantomData)
    }
}

impl<T: ?Sized> PartialEq for ID<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: ?Sized> Eq for ID<T> {}

impl<T: ?Sized> Copy for ID<T> {}
