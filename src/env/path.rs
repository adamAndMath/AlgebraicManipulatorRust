#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Path<'f>(Vec<&'f str>);

impl<'f> Path<'f> {
    pub fn new(v: Vec<&'f str>) -> Self {
        Path(v)
    }

    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.0.iter().map(|s|*s)
    }

    pub fn name(&self) -> &'f str {
        self.0[self.0.len() - 1]
    }
}

impl<'f> From<&'f str> for Path<'f> {
    fn from(name: &'f str) -> Self {
        Path(vec![name])
    }
}

impl<'f> AsRef<[&'f str]> for Path<'f> {
    fn as_ref(&self) -> &[&'f str] {
        &self.0
    }
}