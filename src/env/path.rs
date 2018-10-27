#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Path(Vec<String>);

impl Path {
    pub fn new(v: Vec<String>) -> Self {
        Path(v)
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn name(&self) -> String {
        self.0[self.0.len() - 1].clone()
    }

    pub fn prepend(mut self, space: String) -> Self {
        self.0.insert(0, space);
        self
    }

    pub fn append(mut self, space: String) -> Self {
        self.0.push(space);
        self
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