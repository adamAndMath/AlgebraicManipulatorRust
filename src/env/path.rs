#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Path<T>(Vec<T>);

impl<T> Path<T> {
    pub fn new(v: Vec<T>) -> Self {
        Path(v)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }

    pub fn name(&self) -> &T {
        &self.0[self.0.len() - 1]
    }
}

impl<T> From<T> for Path<T> {
    fn from(s: T) -> Self {
        Path(vec![s])
    }
}

impl<T> AsRef<[T]> for Path<T> {
    fn as_ref(&self) -> &[T] {
        &self.0
    }
}