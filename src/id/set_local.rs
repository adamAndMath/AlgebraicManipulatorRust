pub trait SetLocal<T = Self>: Sized {
    fn set_with_min(&self, min: usize, par: &[T]) -> Self;

    fn set(&self, par: &[T]) -> Self {
        self.set_with_min(0, par)
    }
}

impl<T, U: SetLocal<T>> SetLocal<T> for Box<U> {
    fn set_with_min(&self, min: usize, par: &[T]) -> Self {
        Box::new((&**self).set_with_min(min, par))
    }
}

impl<T, U: SetLocal<T>> SetLocal<T> for Vec<U> {
    fn set_with_min(&self, min: usize, par: &[T]) -> Self {
        self.iter().map(|e|e.set_with_min(min, par)).collect()
    }
}
