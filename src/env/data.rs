use super::ID;

#[derive(Debug)]
pub struct EnvData<T> {
    predef: Vec<T>,
    normal: Vec<T>,
}

impl<T> EnvData<T> {
    pub fn new(predef: Vec<T>) -> Self {
        EnvData { predef, normal: vec![] }
    }

    pub fn add(&mut self, e: T) -> ID<T> {
        let id = ID::new(self.normal.len());
        self.normal.push(e);
        id
    }

    pub fn get(&self, id: ID<T>) -> &T {
        match id {
            ID::Predef(id, _) => &self.predef[id],
            ID::Normal(id, _) => &self.normal[id],
        }
    }

    pub fn get_mut(&mut self, id: ID<T>) -> &mut T {
        match id {
            ID::Predef(id, _) => &mut self.predef[id],
            ID::Normal(id, _) => &mut self.normal[id],
        }
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.normal.len()
    }
}

#[cfg(test)]
use std::ops::Index;

#[cfg(test)]
impl<T, I> Index<I> for EnvData<T> where Vec<T>: Index<I> {
    type Output = <Vec<T> as Index<I>>::Output;
    fn index(&self, i: I) -> &Self::Output {
        &self.normal[i]
    }
}