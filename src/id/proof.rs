use predef::*;
use env::{ ID, PushID };
use envs::{ Envs, TruthVal };
use super::{ Type, Pattern, Patterned, Exp, Element, ErrID, SetLocal, TypeCheck };
use tree::Tree;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Forwards,
    Backwards,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RefType {
    Ref(ID<TruthVal>),
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

    pub fn get(&self, env: &Envs) -> Result<Exp, ErrID> {
        match self.id {
            RefType::Ref(id) => env.truth[id].get(id, self.gen.clone(), self.par.clone(), env),
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
                            .filter_map(|Patterned(p,a)|{let v = p.match_exp(arg.clone(), env).ok()?; Some(a.set(&v))})
                            .next()
                            .ok_or(ErrID::NoMatch(arg.clone()))?,
                    _ => unimplemented!(),
                };
                let ty = par.type_check(env)?;
                Ok(Exp::Call(Box::new(Exp::Var(EQ_ID.into(), vec![ty])), Box::new(Exp::Tuple(vec![par.clone(), res]))))
            },
            RefType::Match => {
                let par = self.par.as_ref().ok_or(ErrID::ArgumentAmount(self.id, 1))?.clone();
                let res = env.mtch.get(&par).ok_or(ErrID::NoMatch(par.clone()))?;
                let ty = par.type_check(env)?;
                Ok(Exp::Call(Box::new(Exp::Var(EQ_ID.into(), vec![ty])), Box::new(Exp::Tuple(vec![par.clone(), res]))))
            },
        }
    }

    pub fn apply(&self, dir: Direction, path: &Tree, exp: Exp, env: &Envs) -> Result<Exp, ErrID> {
        let truth = self.get(env)?;
        if let Exp::Call(box Exp::Var(eq, t), box Exp::Tuple(v)) = truth {
            if eq != EQ_ID { return Err(ErrID::ExpMismatch(Exp::Var(eq, vec![]), Exp::Var(EQ_ID.into(), vec![])))}
            if let [ref par, ref res] = v[..] {
                let (par, res) = match dir {
                    Direction::Forwards => (par, res),
                    Direction::Backwards => (res, par),
                };

                exp.apply(path, 0, &|e, i| {
                    let par = par.push_id(i);
                    if *e == par {
                        Ok(res.push_id(i))
                    } else {
                        Err(ErrID::ExpMismatch(e.clone(), par))
                    }
                }).map_err(|e|match e {Ok(e) => e, Err(e) => e.into()})
            } else {Err(ErrID::NoMatch(Exp::Call(Box::new(Exp::Var(EQ_ID, t)), Box::new(Exp::Tuple(v.clone())))))}
        } else {Err(ErrID::NoMatch(truth))}
    }
}

#[derive(Debug)]
pub enum Proof {
    Sequence(TruthRef, Vec<(Direction, TruthRef, Tree)>),
    Block(Vec<Element>, Box<Proof>),
    Match(Exp, Vec<Patterned<Proof>>),
    Forall(Vec<Patterned<Proof>>),
}

impl Proof {
    pub fn execute(&self, env: &Envs) -> Result<Exp, ErrID> {
        Ok(match self {
            Proof::Sequence(initial, rest) => {
                let mut proof = initial.get(env)?;
                for (dir, truth, path) in rest {
                    proof = truth.apply(*dir, path, proof, env)?;
                }
                proof
            },
            Proof::Block(elm, proof) => {
                let mut env = env.scope_empty();
                for elm in elm {
                    elm.define(&mut env)?;
                }
                proof.execute(&env)?.pop_id(1).ok_or(ErrID::NotContained)?
            },
            Proof::Match(e, v) => {
                let mut re: Option<Exp> = None;
                for Patterned(pattern, proof) in v {
                    let p = proof.execute(&env.scope_match(pattern.bound(), expand(0, &e.push_id(1), pattern)?))?.pop_id(1).ok_or(ErrID::NotContained)?;
                    if let Some(re) = &re {
                        if *re != p {
                            return Err(ErrID::ExpMismatch(p, re.clone()));
                        }
                        continue;
                    }
                    re = Some(p);
                }
                re.unwrap()
            },
            Proof::Forall(v) => {
                let mut t_in = None;
                for Patterned(p,_) in v {
                    let t = p.type_check(&env.scope_empty())?;
                    if let Some(ref t_in) = t_in {
                        if t != *t_in { return Err(ErrID::TypeMismatch(t, t_in.clone())); }
                    } else {
                        t_in = Some(t);
                    }
                }
                let t_in = t_in.unwrap().pop_id(1).unwrap();
                let v = v.iter().map(|Patterned(pattern, proof)| Ok(Patterned(pattern.clone(), proof.execute(&env.scope_exp(pattern.bound()))?))).collect::<Result<_,ErrID>>()?;
                let closure = Exp::Closure(v);
                Exp::Call(Box::new(Exp::Var(FORALL_ID.into(), vec![t_in])), Box::new(closure))
            },
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
