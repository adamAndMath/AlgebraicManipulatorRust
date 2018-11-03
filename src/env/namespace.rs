use std::collections::HashMap;
use super::{ ID, Path };

#[derive(Debug)]
pub enum SpaceVal<T> {
    Val(ID<T>),
    Space(Space<T>),
}

#[derive(Debug)]
pub struct Space<T> {
    names: HashMap<String, SpaceVal<T>>,
}

#[derive(Debug)]
struct SpaceRef<'a, T: 'a> {
    space: &'a mut Space<T>,
    local: &'a mut Space<T>,
    base: Option<&'a SpaceRef<'a, T>>,
}

#[derive(Debug)]
pub struct Namespace<'a, T: 'a> {
    space: Space<T>,
    local: Space<T>,
    base: Option<SpaceRef<'a, T>>,
    paths: &'a mut Vec<Path<String>>,
    path: Path<String>,
}

impl<T> Clone for SpaceVal<T> {
    fn clone(&self) -> Self {
        match self {
            SpaceVal::Val(id) => SpaceVal::Val(*id),
            SpaceVal::Space(space) => SpaceVal::Space(space.clone()),
        }
    }
}

impl<T> Clone for Space<T> {
    fn clone(&self) -> Self {
        Space { names: self.names.clone() }
    }
}

impl<T> Default for Space<T> {
    fn default() -> Self {
        Space { names: HashMap::new() }
    }
}

impl<T> Space<T> {
    pub fn new<S: AsRef<str>, I: IntoIterator<Item = (S, ID<T>)>>(predef: I) -> Self {
        Space {
            names: predef.into_iter().map(|(n, id)|(n.as_ref().to_owned(), SpaceVal::Val(id))).collect(),
        }
    }

    pub fn add<S: AsRef<str>>(&mut self, n: &S, val: SpaceVal<T>) {
        self.names.insert(n.as_ref().to_owned(), val);
    }

    pub fn get<S: Clone + AsRef<str>>(&self, p: &[S]) -> Result<&SpaceVal<T>, Path<S>> {
        let s = &p[0];
        let p = &p[1..];
        let v = self.names.get(s.as_ref());
        if p.is_empty() {
            v.ok_or_else(||s.to_owned().into())
        } else {
            if let Some(SpaceVal::Space(space)) = v {
                space.get(p).map_err(|p|p.prepend(s.clone()))
            } else {
                Err(s.to_owned().into())
            }
        }
    }
}

impl<'a, T: 'a> Namespace<'a, T> {
    pub fn new(paths: &'a mut Vec<Path<String>>, space: Space<T>) -> Self {
        Namespace {
            space: space.clone(),
            local: space,
            base: None,
            paths,
            path: Path::new(vec![]),
        }
    }

    pub fn sub_space<'b, S: AsRef<str>>(&'b mut self, n: &S) -> Namespace<'b, T> where 'a: 'b {
        let path = self.path.clone().append(n.as_ref().to_owned());
        Namespace {
            base: Some(SpaceRef {
                space: &mut self.space,
                local: &mut self.local,
                base: self.base.as_ref()
            }),
            space: Space { names: HashMap::new() },
            local: Space { names: HashMap::new() },
            paths: self.paths,
            path,
        }
    }

    pub fn alias<S: Clone + AsRef<str>>(&mut self, n: &S, p: &Path<S>) -> Result<(), Path<S>> {
        let val = self.get_val(p.as_ref())?.clone();
        self.local.add(n, val);
        Ok(())
    }

    pub fn add<S: AsRef<str>>(&mut self, name: &S) -> ID<T> {
        let id = ID::new(self.paths.len());
        self.local.add(name, SpaceVal::Val(id));
        self.space.add(name, SpaceVal::Val(id));
        self.paths.push(self.path.clone().append(name.as_ref().to_owned()));
        id
    }

    fn get_val<S: Clone + AsRef<str>>(&self, path: &[S]) -> Result<&SpaceVal<T>, Path<S>> {
        if path[0].as_ref() == "super" {
            let mut i = 1;
            let mut s = self.base.as_ref().ok_or_else(||Path::new(path[..i].to_owned()))?;
            while path[i].as_ref() == "super" {
                i += 1;
                s = s.base.ok_or_else(||Path::new(path[..i].to_owned()))?;
            }

            s.space.get(&path[i..])
        } else {
            self.local.get(path)
        }
    }

    pub fn get<S: Clone + AsRef<str>>(&self, p: &Path<S>) -> Result<ID<T>, Path<S>> {
        match self.get_val(p.as_ref())? {
            SpaceVal::Val(id) => Ok(*id),
            SpaceVal::Space(_) => Err(p.clone())
        }
    }
}

impl<'a, T: 'a> Drop for Namespace<'a, T> {
    fn drop(&mut self) {
        if let Some(ref mut base) = self.base {
            base.space.add(self.path.name(), SpaceVal::Space(self.space.clone()));
            base.local.add(self.path.name(), SpaceVal::Space(self.space.clone()));
        }
    }
}
