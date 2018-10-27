use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use super::{ ID, Path };

#[derive(Debug)]
pub struct Namespace<T> {
    names: HashMap<String, SpaceVal<T>>,
    path: Path,
    paths: Rc<RefCell<Vec<Path>>>,
}

#[derive(Debug)]
pub enum SpaceVal<T> {
    Val(ID<T>),
    Space(Namespace<T>)
}

impl<T> Namespace<T> {
    pub fn new<I: IntoIterator<Item = (String, ID<T>)>>(predef: I) -> Self {
        let paths = Rc::new(RefCell::new(vec![]));
        Namespace { names: predef.into_iter().map(|(n, id)|(n, SpaceVal::Val(id))).collect(), path: Path::new(vec![]), paths }
    }

    pub fn sub_space(&self, n: String) -> Namespace<T> {
        Namespace { names: HashMap::new(), path: self.path.clone().append(n) , paths: self.paths.clone() }
    }

    pub fn alias(&mut self, n: String, p: &Path) -> Result<(), Path> {
        let val = self.get_val(p.as_ref())?.clone();
        self.names.insert(n, val);
        Ok(())
    }

    pub fn add(&mut self, name: String) -> ID<T> {
        let id = ID::new(self.paths().len());
        self.names.insert(name.clone(), SpaceVal::Val(id));
        self.paths.borrow_mut().push(self.path.clone().append(name));
        id
    }

    pub fn add_space(&mut self, name: String, space: Namespace<T>) {
        self.names.insert(name, SpaceVal::Space(space));
    }

    fn get_val(&self, p: &[String]) -> Result<&SpaceVal<T>, Path> {
        let s = &p[0];
        let p = &p[1..];
        let v = self.names.get(s);
        if p.is_empty() {
            v.ok_or_else(||s.to_owned().into())
        } else {
            if let Some(SpaceVal::Space(space)) = v {
                space.get_val(p)
            } else {
                Err(s.to_owned().into())
            }
        }
    }

    pub fn get(&self, p: &Path) -> Result<ID<T>, Path> {
        match self.get_val(p.as_ref())? {
            SpaceVal::Val(id) => Ok(*id),
            SpaceVal::Space(_) => Err(p.clone())
        }
    }

    pub fn paths(&self) -> Vec<Path> {
        self.paths.borrow().clone()
    }
}

impl<T> Clone for Namespace<T> {
    fn clone(&self) -> Self {
        Namespace {
            names: self.names.clone(),
            path: self.path.clone(),
            paths: self.paths.clone(),
        }
    }
}

impl<T> Clone for SpaceVal<T> {
    fn clone(&self) -> Self {
        match self {
            SpaceVal::Val(id) => SpaceVal::Val(*id),
            SpaceVal::Space(space) => SpaceVal::Space(space.clone()),
        }
    }
}
