use std::fmt::{ self, Display, Formatter };

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Path<T>(Vec<T>);

impl<T> Path<T> {
    pub fn new(v: Vec<T>) -> Self {
        Path(v)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn name(&self) -> &T {
        &self.0[self.0.len() - 1]
    }

    pub fn prepend(mut self, space: T) -> Self {
        self.0.insert(0, space);
        self
    }

    pub fn append(mut self, space: T) -> Self {
        self.0.push(space);
        self
    }
}

impl<T: Display> Display for Path<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut s = "";
        for e in &self.0 {
            write!(f, "{}{}", s, e)?;
            s = "::";
        }
        Ok(())
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