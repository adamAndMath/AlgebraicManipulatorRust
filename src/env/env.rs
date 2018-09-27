use std::collections::HashMap;
use std::borrow::Borrow;
use std::hash::Hash;
use super::ID;

#[derive(Debug)]
pub struct Env<'a, T: 'a>(HashMap<String, ID<T>>, &'a mut Vec<T>);

impl<'a, T: 'a> Env<'a, T> {
    pub fn new(v: &'a mut Vec<T>) -> Self {
        Env(HashMap::new(), v)
    }

    pub fn scope<'b>(&'b mut self) -> Env<'b, T> where 'a: 'b {
        Env(self.0.clone(), &mut self.1)
    }

    pub fn add(&mut self, name: String, element: T) -> ID<T> {
        let id = ID::new(self.1.len());
        self.0.insert(name, id);
        self.1.push(element);
        id
    }

    pub fn alias(&mut self, name: String, id: ID<T>) {
        self.0.insert(name, id);
    }

    pub fn get_id<S: ?Sized + Hash + Eq + AsRef<str>>(&self, name: &S) -> Result<ID<T>, String> where String: Borrow<S> {
        self.0.get(name).cloned().ok_or(name.as_ref().to_owned())
    }

    pub fn get(&self, id: ID<T>) -> Result<&T, ID<T>> {
        self.1.get(id.0).ok_or(id)
    }

    pub fn get_mut(&mut self, id: ID<T>) -> Result<&mut T, ID<T>> {
        self.1.get_mut(id.0).ok_or(id)
    }
}