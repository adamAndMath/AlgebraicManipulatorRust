use env::{ LocalID, PushLocal };
use super::{ Type, Pattern, ErrID, TypeCheck, TypeCheckIter };
use envs::{ ExpVal, LocalEnvs };
use tree::{Tree, TreeChar };

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Exp {
    Var(LocalID<ExpVal>, Vec<Type>),
    Tuple(Vec<Exp>),
    Lambda(Pattern, Box<Exp>),
    Call(Box<Exp>, Box<Exp>),
    Match(Box<Exp>, Vec<(Pattern, Exp)>),
}

impl TypeCheck for Exp {
    fn type_check(&self, env: &LocalEnvs) -> Result<Type, ErrID> {
        Ok(match self {
            Exp::Var(x, gs) => env.exp.get(*x)?.ty(),
            Exp::Tuple(v) => Type::Tuple(v.type_check(env)?),
            Exp::Lambda(p, e) => (p, e).type_check(env)?,
            Exp::Call(f, e) => f.type_check(env)?.call_output(&e.type_check(env)?)?,
            Exp::Match(e, ps) => {
                let e = e.type_check(env)?;
                let mut op: Option<Type> = None;
                for p in ps.type_check(env)? {
                    let t = p.call_output(&e)?;
                    if let Some(ref ty) = op {
                        if t != *ty { return Err(ErrID::TypeMismatch(t, ty.clone())) }
                    } else {
                        op = Some(t)
                    }
                }
                op.unwrap()
            },
        })
    }
}

impl PushLocal for Exp {
    fn push_local_with_min(&self, min: usize, amount: usize) -> Self {
        match self {
            Exp::Var(id, ty) => Exp::Var(id.push_local_with_min(min, amount), ty.clone()),
            Exp::Tuple(v) => Exp::Tuple(v.push_local_with_min(min, amount)),
            Exp::Lambda(p, e) => Exp::Lambda(p.clone(), e.push_local_with_min(min + p.bounds(), amount)),
            Exp::Call(f, e) => Exp::Call(f.push_local_with_min(min, amount), e.push_local_with_min(min, amount)),
            Exp::Match(e, v) => Exp::Match(e.push_local_with_min(min, amount), v.push_local_with_min(min, amount)),
        }
    }
}

impl Exp {
    pub fn set(&self, par: &[Self]) -> Self {
        self.set_with_min(0, par)
    }

    fn set_with_min(&self, min: usize, par: &[Self]) -> Self {
        match self {
            Exp::Var(LocalID::Local(id, _), ty) => {
                if *id < min {
                    Exp::Var(LocalID::new(*id), ty.clone())
                } else if id - min >= par.len() {
                    Exp::Var(LocalID::new(id - par.len()), ty.clone())
                } else {
                    par[id - min].push_local(min)
                }
            },
            Exp::Var(id, ty) => Exp::Var(*id, ty.clone()),
            Exp::Tuple(v) => Exp::Tuple(v.into_iter().map(|e|e.set_with_min(min, par)).collect()),
            Exp::Lambda(p, e) => Exp::Lambda(p.clone(), Box::new(e.set_with_min(min + p.bounds(), par))),
            Exp::Call(f, e) => Exp::Call(Box::new(f.set_with_min(min, par)), Box::new(e.set_with_min(min, par))),
            Exp::Match(e, v) => Exp::Match(Box::new(e.set_with_min(min, par)), v.into_iter().map(|(p, e)|(p.clone(), e.set_with_min(min + p.bounds(), par))).collect()),
        }
    }

    pub fn apply<E, F: Fn(&Self, usize) -> Result<Self, E>>(&self, path: &Tree, i: usize, f: &F) -> Result<Self, Result<E, Tree>> {
        if path.is_empty() {
            f(self, i).map_err(Ok)
        } else {
            Ok(match self {
                Exp::Var(_, _) => return Err(Err(path.clone())),
                Exp::Tuple(v) => {
                    path.is_within(0..v.len(), &[]).map_err(Err)?;

                    Exp::Tuple(v.into_iter().enumerate().map(|(i, e)|
                        match path.get(i) {
                            Some(p) => e.apply(p, i, f).map_err(|e|e.map_err(|t|Tree::edge(i)+t)),
                            None => Ok(e.clone()),
                        }
                    ).collect::<Result<_,_>>()?)
                },
                Exp::Lambda(p, e) => {
                    path.is_within(0..1, &[]).map_err(Err)?;

                    Exp::Lambda(p.clone(), Box::new(
                        match path.get(0) {
                            Some(path) => e.apply(path, i + p.bounds(), f).map_err(|e|e.map_err(|t|Tree::edge(i)+t))?,
                            None => unreachable!(),
                        }
                    ))
                },
                Exp::Call(e1, box e2) => {
                    Exp::Call(
                        match path.get(TreeChar::Func) {
                            Some(p) => Box::new(e1.apply(p, i, f).map_err(|e|e.map_err(|t|Tree::edge(i)+t))?),
                            None => e1.clone(),
                        },
                        Box::new(
                            if let Err(outside) = path.is_within(0..0, &[TreeChar::Func, TreeChar::Tuple]) {
                                match e2 {
                                    Exp::Tuple(v) => {
                                        path.is_within(0..v.len(), &[TreeChar::Func]).map_err(|t|Err(outside*t))?;

                                        Exp::Tuple(v.into_iter().enumerate().map(|(i, e)|
                                            match path.get(i) {
                                                Some(p) => e.apply(p, i, f).map_err(|e|e.map_err(|t|Tree::edge(i)+t)),
                                                None => Ok(e.clone()),
                                            }
                                        ).collect::<Result<_,_>>()?)
                                    },
                                    e => {
                                        path.is_within(0..1, &[TreeChar::Func]).map_err(|t|Err(outside*t))?;
                                        match path.get(0) {
                                            Some(p) => e.apply(p, i, f).map_err(|e|e.map_err(|t|Tree::edge(i)+t))?,
                                            None => e.clone(),
                                        }
                                    }
                                }
                            } else {
                                match path.get(TreeChar::Tuple) {
                                    Some(p) => e2.apply(p, i, f).map_err(|e|e.map_err(|t|Tree::edge(i)+t))?,
                                    None => (e2).clone(),
                                }
                            }
                        )
                    )
                },
                Exp::Match(e, v) => {
                    path.is_within(0..v.len(), &[TreeChar::Func]).map_err(Err)?;

                    Exp::Match(
                        match path.get(TreeChar::Func) {
                            Some(p) => Box::new(e.apply(p, i, f).map_err(|e|e.map_err(|t|Tree::edge(i)+t))?),
                            None => e.clone(),
                        },
                        v.into_iter().enumerate().map(|(i, (p, e))|
                            match path.get(i) {
                                Some(path) => e.apply(path, i, f).map(|e|(p.clone(), e)).map_err(|e|e.map_err(|t|Tree::edge(i)+t)),
                                None => Ok((p.clone(), e.clone())),
                            }
                        ).collect::<Result<_,_>>()?
                    )
                }
            })
        }
    }
}
