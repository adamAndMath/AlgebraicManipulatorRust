use std::collections::HashMap;
use super::{ ID, Path, Val };

#[derive(Debug)]
pub struct Env<'a, T: 'a> {
    vals: HashMap<String, Val<T>>,
    space: HashMap<String, Val<T>>,
    data: &'a mut Vec<T>,
}

impl<'a, T: 'a> Env<'a, T> {
    pub fn new(data: &'a mut Vec<T>) -> Self {
        Env {
            vals: HashMap::new(),
            space: HashMap::new(),
            data
        }
    }

    pub fn child_scope<'b>(&'b mut self) -> Env<'b, T> where 'a: 'b {
        let mut vals = HashMap::new();
        vals.insert("super".to_owned(), Val::Space(self.space.clone()));

        Env {
            vals,
            space: HashMap::new(),
            data: &mut self.data,
        }
    }

    pub fn to_val(self) -> Val<T> {
        Val::Space(self.space)
    }

    pub fn add(&mut self, name: &str, element: T) -> ID<T> {
        let id = ID::new(self.data.len());
        self.vals.insert(name.to_owned(), Val::ID(id));
        self.space.insert(name.to_owned(), Val::ID(id));
        self.data.push(element);
        id
    }

    pub fn add_val(&mut self, name: &str, val: Val<T>) {
        self.vals.insert(name.to_owned(), val.clone());
        self.space.insert(name.to_owned(), val);
    }

    pub fn alias(&mut self, name: &str, val: Val<T>) {
        self.vals.insert(name.to_owned(), val);
    }

    pub fn get_val<'f>(&self, path: &Path<'f>) -> Result<&Val<T>, Path<'f>> {
        let mut iter = path.iter();
        let v = iter.next().and_then(|p|self.vals.get(p)).ok_or_else(||path.clone())?;
        iter.try_fold(v, |v, p| match v {
            Val::ID(_) => Err(path.clone()),
            Val::Space(m) => m.get(p).ok_or_else(||path.clone()),
        })
    }

    pub fn get_id<'f>(&self, path: &Path<'f>) -> Result<ID<T>, Path<'f>> {
        match self.get_val(path)? {
            Val::ID(id) => Ok(*id),
            Val::Space(_) => Err(path.clone())
        }
    }

    pub fn get(&self, id: ID<T>) -> Result<&T, ID<T>> {
        self.data.get(id.0).ok_or(id)
    }

    pub fn get_mut(&mut self, id: ID<T>) -> Result<&mut T, ID<T>> {
        self.data.get_mut(id.0).ok_or(id)
    }
}