#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Path(Vec<String>);

impl Path {
    pub fn new(v: Vec<String>) -> Self {
        Path(v)
    }
}

impl From<String> for Path {
    fn from(name: String) -> Self {
        Path(vec![name])
    }
}

impl AsRef<[String]> for Path {
    fn as_ref(&self) -> &[String] {
        &self.0
    }
}