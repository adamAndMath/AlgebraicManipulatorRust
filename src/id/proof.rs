use std::marker::PhantomData;
use predef::*;
use env::{ LocalID, PushLocal };
use envs::{ LocalEnvs, ExpVal, TruthVal };
use super::{ Type, Pattern, Exp, ErrID, SetLocal, TypeCheck };
use tree::Tree;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Forwards,
    Backwards,
}

pub enum MatchEnv<'a> {
    Base(),
    Extended(&'a MatchEnv<'a>, Vec<(Exp, Exp)>),
}

impl<'a> MatchEnv<'a> {
    pub fn new() -> Self {
        MatchEnv::Base()
    }

    pub fn scope<'b>(&'b self, v: Vec<(Exp, Exp)>) -> MatchEnv<'b> {
        MatchEnv::Extended(&self, v)
    }

    pub fn get(&self, k: &Exp) -> Option<Exp> {
        match self {
            MatchEnv::Base() => None,
            MatchEnv::Extended(b, v) => v.into_iter().filter(|(i,_)|i==k).map(|(_,v)|v.clone()).next().or_else(||b.get(k)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RefType {
    Ref(LocalID<TruthVal>),
    Def,
    Match,
}

#[derive(Debug, Clone)]
pub struct TruthRef {
    id: RefType,
    gen: Vec<Type>,
    par: Option<Exp>,
}

impl TruthRef {
    pub fn new(id: RefType, gen: Vec<Type>, par: Option<Exp>) -> Self {
        TruthRef { id, gen, par }
    }

    pub fn get(&self, env: &LocalEnvs, match_env: &MatchEnv) -> Result<Exp, ErrID> {
        match self.id {
            RefType::Ref(id) => env.truth[id].get(self.gen.clone(), self.par.clone(), env),
            RefType::Def => {
                let par = self.par.as_ref().ok_or(ErrID::ArgumentAmount(self.id, 1))?;
                let res = match par {
                    Exp::Var(id, gs) => env.exp[*id].val(*id, gs)?,
                    Exp::Call(box f, box arg) =>
                        match f {
                            Exp::Var(id, gs) =>
                                match env.exp[*id].val(*id, gs)? {
                                    Exp::Closure(v) => v,
                                    _ => unimplemented!(),
                                },
                            Exp::Closure(v) => v.clone(),
                            _ => unimplemented!(),
                        }.into_iter()
                            .filter_map(|(p,a)|{let v = p.match_exp(arg.clone(), env).ok()?; Some(a.set(&v))})
                            .next()
                            .ok_or(ErrID::NoMatch(arg.clone()))?,
                    _ => unimplemented!(),
                };
                let ty = par.type_check(env)?;
                Ok(Exp::Call(Box::new(Exp::Var(EQ_ID.into(), vec![ty])), Box::new(Exp::Tuple(vec![par.clone(), res]))))
            },
            RefType::Match => {
                let par = self.par.as_ref().ok_or(ErrID::ArgumentAmount(self.id, 1))?.clone();
                let res = match_env.get(&par).ok_or(ErrID::NoMatch(par.clone()))?;
                let ty = par.type_check(env)?;
                Ok(Exp::Call(Box::new(Exp::Var(EQ_ID.into(), vec![ty])), Box::new(Exp::Tuple(vec![par.clone(), res]))))
            },
        }
    }

    pub fn apply(&self, dir: Direction, path: &Tree, exp: Exp, env: &LocalEnvs, match_env: &MatchEnv) -> Result<Exp, ErrID> {
        let truth = self.get(env, match_env)?;
        if let Exp::Call(box Exp::Var(eq, t), box Exp::Tuple(v)) = truth {
            if eq != EQ_ID { return Err(ErrID::ExpMismatch(Exp::Var(eq, vec![]), Exp::Var(EQ_ID.into(), vec![])))}
            if let [ref par, ref res] = v[..] {
                let (par, res) = match dir {
                    Direction::Forwards => (par, res),
                    Direction::Backwards => (res, par),
                };

                exp.apply(path, 0, &|e, i| {
                    let par = par.push_local(PhantomData::<ExpVal>, i);
                    if *e == par {
                        Ok(res.push_local(PhantomData::<ExpVal>, i))
                    } else {
                        Err(ErrID::ExpMismatch(e.clone(), par))
                    }
                }).map_err(|e|match e {Ok(e) => e, Err(e) => e.into()})
            } else {Err(ErrID::NoMatch(Exp::Call(Box::new(Exp::Var(LocalID::Global(EQ_ID), t)), Box::new(Exp::Tuple(v.clone())))))}
        } else {Err(ErrID::NoMatch(truth))}
    }
}

#[derive(Debug, Clone)]
pub enum Proof {
    Sequence(TruthRef, Vec<(Direction, TruthRef, Tree)>),
    Block(Box<Proof>, Box<Proof>),
    Match(Exp, Vec<(Pattern, Proof)>),
}

impl Proof {
    pub fn execute(&self, env: &LocalEnvs, match_env: &MatchEnv) -> Result<Exp, ErrID> {
        Ok(match self {
            Proof::Sequence(initial, rest) => {
                let mut proof = initial.get(env, match_env)?;
                for (dir, truth, path) in rest {
                    proof = truth.apply(*dir, path, proof, env, match_env)?;
                }
                proof
            },
            Proof::Block(def, proof) => {
                let def = def.execute(env, match_env)?;
                proof.execute(&env.scope_truth(vec![TruthVal::new(def, 0)]), match_env)?
            }
            Proof::Match(e, v) => {
                let mut re: Option<Exp> = None;
                for (pattern, proof) in v {
                    let p = proof.execute(env, &match_env.scope(expand(0, &e.push_local(PhantomData::<ExpVal>, pattern.bounds()), pattern)?))?;
                    if let Some(re) = &re {
                        if *re != p {
                            return Err(ErrID::ExpMismatch(p, re.clone()));
                        }
                        continue;
                    }
                    re = Some(p);
                }
                re.unwrap()
            }
        })
    }
}

fn expand(i: usize, e: &Exp, p: &Pattern) -> Result<Vec<(Exp, Exp)>, ErrID> {
    let mut v = vec![(e.clone(), p.to_exp(i))];
    if let (Exp::Tuple(es), Pattern::Tuple(ps)) = (e, p) {
        if es.len() != ps.len() { unreachable!("This should be caught by type checker"); }
        let mut i = i;
        for (e, p) in es.into_iter().zip(ps) {
            let b = p.bounds();
            v.extend(expand(i, e, p)?);
            i += b;
        }
    }
    Ok(v)
}
