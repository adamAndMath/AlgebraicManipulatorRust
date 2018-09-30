pub trait PushLocal: Sized {
    fn push_local_with_min(&self, min: usize, amount: usize) -> Self;

    fn push_local(&self, amount: usize) -> Self {
        self.push_local_with_min(0, amount)
    }
}

impl<T: PushLocal> PushLocal for Box<T> {
    fn push_local_with_min(&self, min: usize, amount: usize) -> Self {
        Box::new((&**self).push_local_with_min(min, amount))
    }
}

impl<T: PushLocal> PushLocal for Vec<T> {
    fn push_local_with_min(&self, min: usize, amount: usize) -> Self {
        self.into_iter().map(|e|e.push_local_with_min(min, amount)).collect()
    }
}
