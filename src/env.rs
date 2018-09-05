use std::collections::HashMap;
use std::borrow::Borrow;
use std::hash::Hash;
use local_env::LocalEnv;

#[derive(Debug)]
pub struct Env<'a, T: 'a>(HashMap<String, usize>, &'a mut Vec<T>);

impl<'a, T: 'a> Env<'a, T> {
    pub fn new(v: &'a mut Vec<T>) -> Self {
        Env(HashMap::new(), v)
    }

    pub fn scope<'b>(&'b mut self) -> Env<'b, T> where 'a: 'b {
        Env(self.0.clone(), &mut self.1)
    }

    pub fn local<'b>(&'b self) -> LocalEnv<'b, T> where 'a: 'b {
        LocalEnv::new(&self)
    }

    pub fn add(&mut self, name: String, element: T) -> usize {
        let id = self.1.len();
        self.0.insert(name, id);
        self.1.push(element);
        id
    }

    pub fn alias(&mut self, name: String, id: usize) {
        self.0.insert(name, id);
    }

    pub fn get_id<S: ?Sized + Hash + Eq>(&self, name: &S) -> Option<&usize> where String: Borrow<S> {
        self.0.get(name)
    }

    pub fn get(&self, id: usize) -> Option<&T> {
        self.1.get(id)
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut T> {
        self.1.get_mut(id)
    }
}