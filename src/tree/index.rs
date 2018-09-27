use std::fmt::{ self, Display, Formatter };

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TreeChar {
    Func,
    Tuple,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TreeIndex {
    Char(TreeChar),
    Index(usize),
}

impl Display for TreeChar {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            TreeChar::Func => write!(f, "f"),
            TreeChar::Tuple => write!(f, "t"),
        }
    }
}

impl Display for TreeIndex {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            TreeIndex::Char(c) => c.fmt(f),
            TreeIndex::Index(i) => i.fmt(f),
        }
    }
}

impl From<TreeChar> for TreeIndex {
    fn from(c: TreeChar) -> Self {
        TreeIndex::Char(c)
    }
}

impl From<usize> for TreeIndex {
    fn from(i: usize) -> Self {
        TreeIndex::Index(i)
    }
}
