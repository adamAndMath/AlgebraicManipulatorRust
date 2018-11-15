use super::{ ID, Path, Space, SpaceVal, PushID };

#[derive(Debug)]
pub struct Namespace<'a, T: 'a> {
    space: Space<T>,
    local: Space<T>,
    parent_space: Option<ParentSpace<'a, T>>,
    parent: Option<&'a Namespace<'a, T>>,
    paths: &'a mut Vec<Path<String>>,
    path: Path<String>,
}

#[derive(Debug)]
pub struct ParentSpace<'a, T: 'a> {
    space: &'a mut Space<T>,
    local: &'a mut Space<T>,
    parent: Option<&'a ParentSpace<'a, T>>,
}

impl<'a, T: 'a> Namespace<'a, T> {
    pub fn new(paths: &'a mut Vec<Path<String>>, space: Space<T>) -> Self {
        Namespace {
            space: space.clone(),
            local: space,
            parent_space: None,
            parent: None,
            paths,
            path: Path::new(vec![]),
        }
    }

    pub fn sub_space<'b, S: AsRef<str>>(&'b mut self, n: &S) -> Namespace<'b, T> where 'a: 'b {
        let path = self.path.clone().append(n.as_ref().to_owned());
        Namespace {
            parent_space: Some(ParentSpace {
                space: &mut self.space,
                local: &mut self.local,
                parent: self.parent_space.as_ref(),
            }),
            parent: self.parent.clone(),
            space: Space::default(),
            local: Space::default(),
            paths: self.paths,
            path,
        }
    }

    pub fn scope<'b, I: IntoIterator>(&'b self, paths: &'a mut Vec<Path<String>>, v: I) -> Namespace<'b, T> where 'a: 'b, I::Item: AsRef<str> {
        let space = Space::new(v.into_iter().enumerate().map(|(i,s)|(s, ID::<T>::new(i))));
        Namespace {
            parent_space: None,
            parent: Some(self),
            local: space.clone(),
            space,
            paths,
            path: Path::new(vec![]),
        }
    }

    pub fn scope_empty<'b>(&'b self, paths: &'a mut Vec<Path<String>>) -> Namespace<'b, T> where 'a: 'b {
        Namespace {
            parent_space: None,
            parent: Some(self),
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
            let mut i = 1;
            let mut s = self.parent_space.as_ref().ok_or_else::<Path<S>,_>(||path[0].to_owned().into())?;

            while path[i].as_ref() == "super" {
                i += 1;
                s = s.parent.ok_or_else(||Path::new(path[..i].to_owned()))?;
            }

            s.space.get(&path[1..]).map(|v|(v,0)).map_err(|p|path[..i].iter().fold(p, |p,s|p.prepend(s.to_owned())))
        } else {
            match self.local.get(path) {
                Ok(v) => Ok((v,0)),
                Err(p) =>
                    if p.len() == 1 {
                        self.parent.ok_or(p)?.get_val(path).map(|(v,i)|(v,i+1))
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
        if let Some(ref mut space) = self.parent_space {
            space.space.add(self.path.name(), SpaceVal::Space(self.space.clone()));
            space.local.add(self.path.name(), SpaceVal::Space(self.space.clone()));
        }
    }
}
