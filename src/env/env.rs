use std::collections::HashMap;
use super::{ ID, Path, Val };

#[derive(Debug)]
pub struct Env<'a, T: 'a>(HashMap<String, Val<T>>, &'a mut Vec<T>);

impl<'a, T: 'a> Env<'a, T> {
    pub fn new(v: &'a mut Vec<T>) -> Self {
        Env(HashMap::new(), v)
    }

    pub fn child_scope<'b>(&'b mut self) -> Env<'b, T> where 'a: 'b {
        Env(HashMap::new(), &mut self.1)
    }

    pub fn scope<'b>(&'b mut self) -> Env<'b, T> where 'a: 'b {
        Env(self.0.clone(), &mut self.1)
    }

    pub fn add(&mut self, name: String, element: T) -> ID<T> {
        let id = ID::new(self.1.len());
        self.0.insert(name.into(), Val::ID(id));
        self.1.push(element);
        id
    }

    pub fn alias(&mut self, name: String, val: Val<T>) {
        self.0.insert(name.into(), val);
    }

    pub fn get_val(&self, path: &Path) -> Result<&Val<T>, Path> {
        let mut iter = path.iter();
        let v = iter.next().and_then(|p|self.0.get(p)).ok_or_else(||path.clone())?;
        iter.try_fold(v, |v, p| match v {
            Val::ID(_) => Err(path.clone()),
            Val::Space(m) => m.get(p).ok_or_else(||path.clone()),
        })
    }

    pub fn get_id(&self, path: &Path) -> Result<ID<T>, Path> {
        match self.get_val(path)? {
            Val::ID(id) => Ok(*id),
            Val::Space(_) => Err(path.clone())
        }
    }

    pub fn get(&self, id: ID<T>) -> Result<&T, ID<T>> {
        self.1.get(id.0).ok_or(id)
    }

    pub fn get_mut(&mut self, id: ID<T>) -> Result<&mut T, ID<T>> {
        self.1.get_mut(id.0).ok_or(id)
    }
}