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

    pub fn add<S: AsRef<str>>(&mut self, name: S, element: T) -> ID<T> {
        let id = self.data.add(element);
        self.vals.insert(name.as_ref().to_owned(), Val::ID(id));
        self.space.insert(name.as_ref().to_owned(), Val::ID(id));
        id
    }

    pub fn add_val<S: AsRef<str>>(&mut self, name: S, val: Val<T>) {
        self.vals.insert(name.as_ref().to_owned(), val.clone());
        self.space.insert(name.as_ref().to_owned(), val);
    }

    pub fn alias<S: AsRef<str>>(&mut self, name: S, val: Val<T>) {
        self.vals.insert(name.as_ref().to_owned(), val);
    }

    pub fn get_val<'f, S: Clone + AsRef<str>>(&self, path: &Path<S>) -> Result<&Val<T>, Path<S>> {
        let mut iter = path.iter();
        let v = iter.next().and_then(|p|self.vals.get(p.as_ref())).ok_or_else(||path.clone())?;
        iter.try_fold(v, |v, p| match v {
            Val::ID(_) => Err(path.clone()),
            Val::Space(m) => m.get(p.as_ref()).ok_or_else(||path.clone()),
        })
    }

    pub fn get_id<'f, S: Clone + AsRef<str>>(&self, path: &Path<S>) -> Result<ID<T>, Path<S>> {
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