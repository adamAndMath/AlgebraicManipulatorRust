use std::collections::HashMap;
use std::borrow::Borrow;
use std::hash::Hash;
use env::Env;
use id::ID;

#[derive(Debug)]
pub enum LocalEnv<'a, T: 'a> {
    Base(&'a Env<'a, T>),
    Scope(&'a LocalEnv<'a, T>, HashMap<String, usize>, Vec<T>),
}

impl<'a, T: 'a> LocalEnv<'a, T> {
    pub fn new(env: &'a Env<'a, T>) -> Self {
        LocalEnv::Base(env)
    }

    pub fn scope<'b>(&'b self, v: Vec<(String, T)>) -> LocalEnv<'b, T> where 'a: 'b {
        LocalEnv::Scope(self, v.iter().enumerate().map(|(id,(n,_))|(n.clone(),id)).collect(), v.into_iter().map(|(_,e)|e).collect())
    }

    pub fn scope_anon<'b>(&'b self, v: Vec<T>) -> LocalEnv<'b, T> where 'a: 'b {
        LocalEnv::Scope(self, HashMap::new(), v)
    }

    pub fn get_id<S: ?Sized + Hash + Eq>(&self, name: &S) -> Option<ID> where String: Borrow<S> {
        match self {
            LocalEnv::Base(env) => env.get_id(name).map(|id|ID::Global(*id)),
            LocalEnv::Scope(env, m, v) =>
                m.get(name).map(|id|ID::Local(*id)).or_else(||
                    Some(match env.get_id(name)? {
                        ID::Global(id) => ID::Global(id),
                        ID::Local(id) => ID::Local(id+v.len()),
                    })
                ),
        }
    }

    pub fn get(&self, id: ID) -> Option<&T> {
        match (self, id) {
            (LocalEnv::Base(env), ID::Global(id)) => env.get(id),
            (LocalEnv::Base(_), ID::Local(_)) => panic!("Unknown ID"),
            (LocalEnv::Scope(env, _, _), ID::Global(id)) => env.get(ID::Global(id)),
            (LocalEnv::Scope(env, _, v), ID::Local(id)) =>
                if v.len() > id {
                    v.get(id)
                } else {
                    env.get(ID::Local(id - v.len()))
                },
        }
    }
}