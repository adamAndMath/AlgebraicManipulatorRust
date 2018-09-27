mod index;

pub use self::index::*;

use std::collections::BTreeMap;
use std::ops::{ Add, Mul, AddAssign, MulAssign, RangeBounds };
use std::fmt::{ self, Display, Formatter };

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Tree {
    map: BTreeMap<TreeIndex, Tree>,
}

impl Tree {
    pub fn edge<I: Into<TreeIndex>>(n: I) -> Self {
        Tree { map: vec![(n.into(), Tree::default())].into_iter().collect() }
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn get<I: Into<TreeIndex>>(&self, i: I) -> Option<&Self> {
        self.map.get(&i.into())
    }

    pub fn is_within<R: RangeBounds<usize>>(&self, range: R, chars: &[TreeChar]) -> Result<(), Tree> {
        let outside = Tree {
            map: self.map.keys().filter(|k|
                match k {
                    TreeIndex::Char(c) => !chars.contains(c),
                    TreeIndex::Index(i) => !range.contains(i),
                }
            ).map(|k|(*k,Tree::default())).collect()
        };

        if outside.is_empty() {
            Ok(())
        } else {
            Err(outside)
        }
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.map.len() == 1 {
            let (k, v) = self.map.iter().next().unwrap();
            if v.map.is_empty() {
                write!(f, "{}", k)
            } else {
                write!(f, "{},{}", k, v)
            }
        } else {
            let mut s = "";
            write!(f, "[")?;
            for (k, v) in &self.map {
                if v.map.is_empty() {
                    write!(f, "{}{}", s, k)?;
                } else {
                    write!(f, "{}{},{}", s, k, v)?;
                }
                s = "|";
            }
            write!(f, "]")
        }
    }
}

impl AddAssign for Tree {
    fn add_assign(&mut self, rhs: Self) {
        if self.map.is_empty() {
            *self = rhs;
        } else {
            for v in self.map.values_mut() {
                *v += rhs.clone();
            }
        }
    }
}

impl MulAssign for Tree {
    fn mul_assign(&mut self, rhs: Self) {
        for (k, v) in rhs.map {
            self.map.entry(k).and_modify(|t|{*t *= v.clone()}).or_insert(v);
        }
    }
}

impl Add for Tree {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

impl Mul for Tree {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self {
        self *= rhs;
        self
    }
}
