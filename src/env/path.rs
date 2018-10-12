#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Path(Vec<String>);

impl Path {
    pub fn new(v: Vec<String>) -> Self {
        Path(v)
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.0.iter()
    }

    pub fn name(&self) -> String {
        self.0[self.0.len() - 1].clone()
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