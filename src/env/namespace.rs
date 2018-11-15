use super::{ ID, Path, Space, SpaceVal, PushID };

#[derive(Debug)]
pub struct Namespace<'a, T: 'a> {
    space: Space<T>,
    local: Space<T>,
    parent: ParentSpace<'a, T>,
    paths: &'a mut Vec<Path<String>>,
    path: Path<String>,
}

#[derive(Debug)]
pub enum ParentSpace<'a, T: 'a> {
    Base,
    Subspace {
        space: &'a mut Space<T>,
        local: &'a mut Space<T>,
        parent: &'a ParentSpace<'a, T>,
    },
    Scope {
        space: &'a Space<T>,
        local: &'a Space<T>,
        parent: &'a ParentSpace<'a, T>,
    }
}

impl<'a, T: 'a> ParentSpace<'a, T> {
    fn get_val<S: Clone + AsRef<str>>(&self, path: &[S]) -> Result<(&SpaceVal<T>, usize), Path<S>> {
        if path[0].as_ref() == "super" {
            let mut i = 0;
            let mut s = self;

            while path[i].as_ref() == "super" {
                i += 1;
                s = self.parent_space().ok_or_else(||Path::new(path[..i].to_owned()))?;
            }

            s.space().ok_or_else(||Path::new(path[..i].to_owned()))?.get(&path[i..]).map(|v|(v,0)).map_err(|p|path[..i].iter().fold(p, |p,s|p.prepend(s.clone())))
        } else {
            match self.local().ok_or_else::<Path<_>,_>(||path[0].to_owned().into())?.get(path) {
                Ok(v) => Ok((v,0)),
                Err(p) =>
                    if p.len() == 1 {
                        self.parent_scope().ok_or(p)?.get_val(path).map(|(v,i)|(v,i+1))
                    } else {
                        Err(p)
                    }
            }
        }
    }

    fn parent_space(&self) -> Option<&Self> {
        match self {
            ParentSpace::Subspace {
                space: _,
                local: _,
                parent,
            } => Some(parent),
            _ => None,
        }
    }

    fn parent_scope(&self) -> Option<&Self> {
        match self {
            ParentSpace::Base => None,
            ParentSpace::Subspace {
                space: _,
                local: _,
                parent,
            } => parent.parent_scope(),
            ParentSpace::Scope {
                space: _,
                local: _,
                parent,
            } => Some(parent),
        }
    }

    fn space(&self) -> Option<&Space<T>> {
        match self {
            ParentSpace::Base => None,
            ParentSpace::Subspace {
                space,
                local: _,
                parent: _,
            } => Some(space),
            ParentSpace::Scope {
                space,
                local: _,
                parent: _,
            } => Some(space),
        }
    }

    fn local(&self) -> Option<&Space<T>> {
        match self {
            ParentSpace::Base => None,
            ParentSpace::Subspace {
                space: _,
                local,
                parent: _,
            } => Some(local),
            ParentSpace::Scope {
                space: _,
                local,
                parent: _,
            } => Some(local),
        }
    }
}

impl<'a, T: 'a> Namespace<'a, T> {
    pub fn new(paths: &'a mut Vec<Path<String>>, space: Space<T>) -> Self {
        Namespace {
            space: space.clone(),
            local: space,
            parent: ParentSpace::Base,
            paths,
            path: Path::new(vec![]),
        }
    }

    pub fn sub_space<'b, S: AsRef<str>>(&'b mut self, n: &S) -> Namespace<'b, T> where 'a: 'b {
        let path = self.path.clone().append(n.as_ref().to_owned());
        Namespace {
            parent: ParentSpace::Subspace {
                space: &mut self.space,
                local: &mut self.local,
                parent: &self.parent,
            },
            space: Space::default(),
            local: Space::default(),
            paths: self.paths,
            path,
        }
    }

    pub fn scope<'b, I: IntoIterator>(&'b self, paths: &'a mut Vec<Path<String>>, v: I) -> Namespace<'b, T> where 'a: 'b, I::Item: AsRef<str> {
        let space = Space::new(v.into_iter().enumerate().map(|(i,s)|(s, ID::<T>::new(i))));
        Namespace {
            parent: ParentSpace::Scope {
                space: &self.space,
                local: &self.local,
                parent: &self.parent,
            },
            local: space.clone(),
            space,
            paths,
            path: Path::new(vec![]),
        }
    }

    pub fn scope_empty<'b>(&'b self, paths: &'a mut Vec<Path<String>>) -> Namespace<'b, T> where 'a: 'b {
        Namespace {
            parent: ParentSpace::Scope {
                space: &self.space,
                local: &self.local,
                parent: &self.parent,
            },
            space: Space::default(),
            local: Space::default(),
            paths,
            path: self.path.clone(),
        }
    }

    pub fn alias<S: Clone + AsRef<str>>(&mut self, n: &S, p: &Path<S>) -> Result<(), Path<S>> {
        let val = {
            let (val, i) = self.get_val(p.as_ref())?;
            val.push_id(i)
        };
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

    fn get_val<S: Clone + AsRef<str>>(&self, path: &[S]) -> Result<(&SpaceVal<T>, usize), Path<S>> {
        if path[0].as_ref() == "super" {
            self.parent.get_val(&path[1..]).map(|(v,i)|(v,i+1)).map_err(|p|p.prepend(path[0].to_owned()))
        } else {
            match self.local.get(path) {
                Ok(v) => Ok((v,0)),
                Err(p) =>
                    if p.len() == 1 {
                        match &self.parent {
                            ParentSpace::Scope {
                                local: _,
                                space: _,
                                parent: _
                            } => &self.parent,
                            _ => self.parent.parent_scope().ok_or(p)?,
                        }.get_val(path).map(|(v,i)|(v,i+1))
                    } else {
                        Err(p)
                    }
            }
        }
    }

    pub fn get<S: Clone + AsRef<str>>(&self, p: &Path<S>) -> Result<ID<T>, Path<S>> {
        match self.get_val(p.as_ref())? {
            (SpaceVal::Val(id), i) => Ok(id.push_id(i)),
            (SpaceVal::Space(_), _) => Err(p.clone()),
        }
    }
}

impl<'a, T: 'a> Drop for Namespace<'a, T> {
    fn drop(&mut self) {
        if let ParentSpace::Subspace { ref mut space, ref mut local, parent: _ } = self.parent {
            space.add(self.path.name(), SpaceVal::Space(self.space.clone()));
            local.add(self.path.name(), SpaceVal::Space(self.space.clone()));
        }
    }
}
