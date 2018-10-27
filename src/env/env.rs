use std::ops::{ Index, IndexMut };
use super::ID;

#[derive(Debug)]
pub struct Env<T> {
    predef: Vec<T>,
    normal: Vec<T>,
}

impl<T> Env<T> {
    pub fn new(predef: Vec<T>) -> Self {
        Env { predef, normal: vec![] }
    }

    pub fn add(&mut self, e: T) -> ID<T> {
        let id = ID::new(self.normal.len());
        self.normal.push(e);
        id
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.normal.len()
    }
}

impl<T> Index<ID<T>> for Env<T> {
    type Output = T;
    fn index(&self, id: ID<T>) -> &T {
        match id {
            ID::Predef(id, _) => &self.predef[id],
            ID::Normal(id, _) => &self.normal[id],
        }
    }
}

impl<T> IndexMut<ID<T>> for Env<T> {
    fn index_mut(&mut self, id: ID<T>) -> &mut T {
        match id {
            ID::Predef(id, _) => &mut self.predef[id],
            ID::Normal(id, _) => &mut self.normal[id],
        }
    }
}

#[cfg(test)]
impl<T, I> Index<I> for Env<T> where Vec<T>: Index<I> {
    type Output = <Vec<T> as Index<I>>::Output;
    fn index(&self, i: I) -> &Self::Output {
        &self.normal[i]
    }
}