use std::marker::PhantomData;
use std::collections::HashMap;
use super::{ Env, LocalID, PushLocal, Path };

#[derive(Debug)]
pub enum LocalEnv<'a, T: 'a> {
    Base(&'a Env<'a, T>),
    Scope(&'a LocalEnv<'a, T>, HashMap<String, usize>, Vec<T>),
}

impl<'a, T: 'a> LocalEnv<'a, T> {
    pub fn new(env: &'a Env<'a, T>) -> Self {
        LocalEnv::Base(env)
    }

    pub fn scope<'b>(&'b self, v: Vec<(&str, T)>) -> LocalEnv<'b, T> where 'a: 'b {
        LocalEnv::Scope(self, v.iter().enumerate().map(|(id,(n,_))|((*n).to_owned(),id)).collect(), v.into_iter().map(|(_,e)|e).collect())
    }

    pub fn scope_anon<'b>(&'b self, v: Vec<T>) -> LocalEnv<'b, T> where 'a: 'b {
        LocalEnv::Scope(self, HashMap::new(), v)
    }

    pub fn get_id<'f>(&self, name: &Path<'f>) -> Result<LocalID<T>, Path<'f>> {
        match self {
            LocalEnv::Base(env) => env.get_id(name).map(|id|id.into()),
            LocalEnv::Scope(env, m, v) => {
                if let [n] = name.as_ref() {
                    if let Some(id) = m.get(*n).map(|id|LocalID::new(*id)) {
                        return Ok(id);
                    }
                }

                env.get_id(name).map(|id|id.push_local(PhantomData::<T>, v.len()))
            },
        }
    }

    pub fn get<I: Into<LocalID<T>>>(&self, id: I) -> &T {
        match (self, id.into()) {
            (LocalEnv::Base(env), LocalID::Global(id)) => env.get(id),
            (LocalEnv::Base(_), LocalID::Local(id, p)) => unreachable!(),
            (LocalEnv::Scope(env, _, _), LocalID::Global(id)) => env.get(LocalID::Global(id)),
            (LocalEnv::Scope(env, _, v), LocalID::Local(id, p)) =>
                if v.len() > id {
                    &v[id]
                } else {
                    env.get(LocalID::Local(id - v.len(), p))
                },
        }
    }
}