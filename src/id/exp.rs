use env::{ ID, PushID };
use super::{ Type, Patterned, ErrID, TypeCheck, TypeCheckIter, SetLocal };
use envs::{ ExpVal, Envs };
use tree::{Tree, TreeChar };

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Exp {
    Var(ID<ExpVal>, Vec<Type>),
    Tuple(Vec<Exp>),
    Closure(Vec<Patterned<Exp>>),
    Call(Box<Exp>, Box<Exp>),
}

impl TypeCheck for Exp {
    fn type_check(&self, env: &Envs) -> Result<Type, ErrID> {
        Ok(match self {
            Exp::Var(x, gs) => env.exp[*x].ty(*x, gs),
            Exp::Tuple(v) => Type::Tuple(v.type_check(env)?),
            Exp::Closure(v) => {
                let mut re: Option<Type> = None;
                for t in v.type_check(env)? {
                    if let Some(ref ty) = re {
                        if t != *ty { return Err(ErrID::TypeMismatch(t, ty.clone())) }
                    } else {
                        re = Some(t)
                    }
                }
                re.unwrap()
            },
            Exp::Call(f, e) => f.type_check(env)?.call_output(&e.type_check(env)?)?,
        })
    }
}

impl PushID for Exp {
    fn push_id_with_min(&self, min: usize, amount: usize) -> Self {
        match self {
            Exp::Var(id, ty) => Exp::Var(id.push_id_with_min(min, amount), ty.push_id_with_min(min, amount)),
            Exp::Tuple(v) => Exp::Tuple(v.push_id_with_min(min, amount)),
            Exp::Closure(v) => Exp::Closure(v.push_id_with_min(min, amount)),
            Exp::Call(f, e) => Exp::Call(f.push_id_with_min(min, amount), e.push_id_with_min(min, amount)),
        }
    }

    fn pop_id_with_min(&self, min: usize, amount: usize) -> Option<Self> {
        Some(match self {
            Exp::Var(id, ty) => Exp::Var(id.pop_id_with_min(min, amount)?, ty.pop_id_with_min(min, amount)?),
            Exp::Tuple(v) => Exp::Tuple(v.pop_id_with_min(min, amount)?),
            Exp::Closure(v) => Exp::Closure(v.pop_id_with_min(min, amount)?),
            Exp::Call(f, e) => Exp::Call(f.pop_id_with_min(min, amount)?, e.pop_id_with_min(min, amount)?),
        })
    }
}

impl SetLocal for Exp {
    fn set_with_min(&self, min: usize, par: &[Self]) -> Self {
        match self {
            Exp::Var(id, ty) => {
                match id.set(min) {
                    Ok(id) => par[id].push_id(min),
                    Err(id) => Exp::Var(id, ty.pop_id_with_min(min, 1).unwrap()),
                }
            }
            Exp::Tuple(v) => Exp::Tuple(v.set_with_min(min, par)),
            Exp::Closure(v) => Exp::Closure(v.set_with_min(min, par)),
            Exp::Call(f, e) => Exp::Call(f.set_with_min(min, par), e.set_with_min(min, par)),
        }
    }
}

impl SetLocal<Type> for Exp {
    fn set_with_min(&self, min: usize, par: &[Type]) -> Self {
        match self {
            Exp::Var(id, ty) => Exp::Var(id.pop_id_with_min(min, 1).unwrap(), ty.set_with_min(min, par)),
            Exp::Tuple(v) => Exp::Tuple(v.set_with_min(min, par)),
            Exp::Closure(v) => Exp::Closure(v.set_with_min(min, par)),
            Exp::Call(f, e) => Exp::Call(f.set_with_min(min, par), e.set_with_min(min, par)),
        }
    }
}

impl Exp {
    pub fn apply<E, F: Fn(&Self, usize) -> Result<Self, E>>(&self, path: &Tree, push: usize, f: &F) -> Result<Self, Result<E, Tree>> {
        if path.is_empty() {
            f(self, push).map_err(Ok)
        } else {
            Ok(match self {
                Exp::Var(_, _) => return Err(Err(path.clone())),
                Exp::Tuple(v) => {
                    path.is_within(0..v.len(), &[]).map_err(Err)?;

                    Exp::Tuple(v.into_iter().enumerate().map(|(i, e)|
                        match path.get(i) {
                            Some(p) => e.apply(p, push, f).map_err(|e|e.map_err(|t|Tree::edge(i)+t)),
                            None => Ok(e.clone()),
                        }
                    ).collect::<Result<_,_>>()?)
                },
                Exp::Closure(v) => {
                    path.is_within(0..v.len(), &[]).map_err(Err)?;

                    Exp::Closure(v.into_iter().enumerate().map(|(i, Patterned(p, e))|
                        match path.get(i) {
                            Some(path) => e.apply(path, push+1, f).map(|e|Patterned(p.clone(), e)).map_err(|e|e.map_err(|t|Tree::edge(i)+t)),
                            None => Ok(Patterned(p.clone(), e.clone())),
                        }
                    ).collect::<Result<_,_>>()?)
                },
                Exp::Call(e1, box e2) => {
                    Exp::Call(
                        match path.get(TreeChar::Func) {
                            Some(p) => Box::new(e1.apply(p, push, f).map_err(|e|e.map_err(|t|Tree::edge(TreeChar::Func)+t))?),
                            None => e1.clone(),
                        },
                        Box::new(
                            if let Err(outside) = path.is_within(0..0, &[TreeChar::Func, TreeChar::Tuple]) {
                                match e2 {
                                    Exp::Tuple(v) => {
                                        path.is_within(0..v.len(), &[TreeChar::Func]).map_err(|t|Err(outside*t))?;

                                        Exp::Tuple(v.into_iter().enumerate().map(|(i, e)|
                                            match path.get(i) {
                                                Some(p) => e.apply(p, push, f).map_err(|e|e.map_err(|t|Tree::edge(i)+t)),
                                                None => Ok(e.clone()),
                                            }
                                        ).collect::<Result<_,_>>()?)
                                    },
                                    e => {
                                        path.is_within(0..1, &[TreeChar::Func]).map_err(|t|Err(outside*t))?;
                                        match path.get(0) {
                                            Some(p) => e.apply(p, push, f).map_err(|e|e.map_err(|t|Tree::edge(0)+t))?,
                                            None => e.clone(),
                                        }
                                    }
                                }
                            } else {
                                match path.get(TreeChar::Tuple) {
                                    Some(p) => e2.apply(p, push, f).map_err(|e|e.map_err(|t|Tree::edge(TreeChar::Tuple)+t))?,
                                    None => (e2).clone(),
                                }
                            }
                        )
                    )
                },
            })
        }
    }
}
