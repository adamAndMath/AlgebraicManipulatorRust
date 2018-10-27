use std::ops::Index;
use super::{ Env, LocalID };

#[derive(Debug)]
pub enum LocalEnv<'a, T: 'a> {
    Base(&'a Env<T>),
    Scope(&'a LocalEnv<'a, T>, Vec<T>),
}

impl<'a, T: 'a> LocalEnv<'a, T> {
    pub fn new(env: &'a Env<T>) -> Self {
        LocalEnv::Base(env)
    }

    pub fn scope<'b>(&'b self, v: Vec<T>) -> LocalEnv<'b, T> where 'a: 'b {
        LocalEnv::Scope(self, v)
    }
}

impl<'a, T, I: Into<LocalID<T>>> Index<I> for LocalEnv<'a, T> {
    type Output = T;
    fn index(&self, id: I) -> &T {
        match (self, id.into()) {
            (LocalEnv::Base(env), LocalID::Global(id)) => &env[id],
            (LocalEnv::Base(_), LocalID::Local(_, _)) => unreachable!(),
            (LocalEnv::Scope(env, _), LocalID::Global(id)) => &env[LocalID::Global(id)],
            (LocalEnv::Scope(env, v), LocalID::Local(id, p)) =>
                if v.len() > id {
                    &v[id]
                } else {
                    &env[LocalID::Local(id - v.len(), p)]
                },
        }
    }
}
