use std::collections::BTreeMap;
use std::fmt::{ self, Display, Formatter };
use std::ops::{ Add, Mul, AddAssign, MulAssign };

#[derive(Debug, Clone, Default)]
pub struct Tree {
    map: BTreeMap<usize, Tree>,
}

impl Tree {
    pub fn edge(n: usize) -> Self {
        Tree { map: vec![(n, Tree::default())].into_iter().collect() }
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
