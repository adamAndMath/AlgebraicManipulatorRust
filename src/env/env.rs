use std::collections::HashMap;
use super::{ ID, Path, Val, EnvData };

#[derive(Debug)]
pub struct Env<'a, T: 'a> {
    vals: HashMap<String, Val<T>>,
    space: HashMap<String, Val<T>>,
    data: &'a mut EnvData<T>,
}

impl<'a, T: 'a> Env<'a, T> {
    pub fn new(data: &'a mut EnvData<T>) -> Self {
        Env {
            vals: HashMap::new(),
            space: HashMap::new(),
            data
        }
    }

    pub fn child_scope<'b>(&'b mut self) -> Env<'b, T> where 'a: 'b {
        Env {
            vals: HashMap::new(),
            space: HashMap::new(),
            data: &mut self.data,
        }
    }

    pub fn to_val(self) -> Val<T> {
        Val::Space(self.space)
    }

    pub fn add(&mut self, name: String, element: T) -> ID<T> {
        let id = self.data.add(element);
        self.vals.insert(name.clone().into(), Val::ID(id));
        self.space.insert(name.into(), Val::ID(id));
        id
    }

    pub fn add_val(&mut self, name: String, val: Val<T>) {
        self.vals.insert(name.clone().into(), val.clone());
        self.space.insert(name.into(), val);
    }

    pub fn alias(&mut self, name: String, val: Val<T>) {
        self.vals.insert(name.into(), val);
    }

    pub fn get_val(&self, path: &Path) -> Result<&Val<T>, Path> {
        let mut iter = path.iter();
        let v = iter.next().and_then(|p|self.vals.get(p)).ok_or_else(||path.clone())?;
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

    pub fn get(&self, id: ID<T>) -> &T {
        self.data.get(id)
    }

    pub fn get_mut(&mut self, id: ID<T>) -> &mut T {
        self.data.get_mut(id)
    }
}