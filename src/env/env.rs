use std::ops::{ Index, IndexMut };
use super::ID;

#[derive(Debug)]
pub struct Env<'a, T: 'a> {
    predef: Result<&'a mut Vec<T>, &'a Vec<T>>,
    data: EnvRef<'a, T>,
}

#[derive(Debug)]
struct EnvRef<'a, T> {
    normal: Vec<T>,
    parent: Option<&'a EnvRef<'a, T>>,
}

impl<'a, T: 'a> Env<'a, T> {
    pub fn new(predef: &'a mut Vec<T>) -> Self {
        Env { predef: Ok(predef), data: EnvRef { normal: vec![], parent: None } }
    }

    pub fn scope<'b>(&'b self, v: Vec<T>) -> Env<'b, T> where 'a: 'b {
        let predef = match &self.predef {
            Ok(e) => Err(&**e),
            Err(e) => Err(*e),
        };
        Env { predef, data: EnvRef { normal: v, parent: Some(&self.data) } }
    }

    pub fn add(&mut self, e: T) -> ID<T> {
        let id = ID::new(self.data.normal.len());
        self.data.normal.push(e);
        id
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.data.normal.len()
    }
}

impl<'a, T: 'a> Index<ID<T>> for Env<'a, T> {
    type Output = T;
    fn index(&self, id: ID<T>) -> &T {
        match id {
            ID::Predef(id, _) =>
                match &self.predef {
                    Ok(e) => &e[id],
                    Err(e) => &e[id],
                },
            ID::Normal(id, up, _) => &(0..up).fold(&self.data, |d,i|d.parent.expect(&format!("Cannot go more than {}, up, tried to go {}.", i, up))).normal[id],
        }
    }
}

impl<'a, T: 'a> IndexMut<ID<T>> for Env<'a, T> {
    fn index_mut(&mut self, id: ID<T>) -> &mut T {
        match id {
            ID::Predef(id, _) => &mut self.predef.as_mut().ok().unwrap()[id],
            ID::Normal(id, 0, _) => &mut self.data.normal[id],
            ID::Normal(_, _, _) => unreachable!(),
        }
    }
}

#[cfg(test)]
impl<'a, T: 'a, I> Index<I> for Env<'a, T> where Vec<T>: Index<I> {
    type Output = <Vec<T> as Index<I>>::Output;
    fn index(&self, i: I) -> &Self::Output {
        &self.data.normal[i]
    }
}