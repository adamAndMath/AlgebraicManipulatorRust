use std::marker::PhantomData;

pub trait PushLocal<T: ?Sized>: Sized {
    fn push_local_with_min(&self, p: PhantomData<T>, min: usize, amount: usize) -> Self;

    fn push_local(&self, p: PhantomData<T>, amount: usize) -> Self {
        self.push_local_with_min(p, 0, amount)
    }
}

impl<T, U: PushLocal<T>, V: PushLocal<T>> PushLocal<T> for (U, V) {
    fn push_local_with_min(&self, p: PhantomData<T>, min: usize, amount: usize) -> Self {
        (self.0.push_local_with_min(p, min, amount), self.1.push_local_with_min(p, min, amount))
    }
}

impl<T, U: PushLocal<T>> PushLocal<T> for Box<U> {
    fn push_local_with_min(&self, p: PhantomData<T>, min: usize, amount: usize) -> Self {
        Box::new((&**self).push_local_with_min(p, min, amount))
    }
}

impl<T, U: PushLocal<T>> PushLocal<T> for Vec<U> {
    fn push_local_with_min(&self, p: PhantomData<T>, min: usize, amount: usize) -> Self {
        self.into_iter().map(|e|e.push_local_with_min(p, min, amount)).collect()
    }
}
