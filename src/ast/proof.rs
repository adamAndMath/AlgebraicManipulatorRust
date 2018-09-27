use envs::LocalEnvs;
use ast::{ Type, Pattern, Exp };
use id::renamed::{ ExpID, PatternID };
use tree::Tree;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Forwards,
    Backwards,
}

#[derive(Debug)]
pub struct TruthRef {
    name: String,
    gen: Vec<Type>,
    par: Vec<Exp>,
}

pub enum MatchEnv<'a> {
    Base(),
    Extended(&'a MatchEnv<'a>, Vec<(ExpID, ExpID)>),
}

impl<'a> MatchEnv<'a> {
    pub fn new() -> Self {
        MatchEnv::Base()
    }

    pub fn scope<'b>(&'b self, v: Vec<(ExpID, ExpID)>) -> MatchEnv<'b> {
        MatchEnv::Extended(&self, v)
    }

    pub fn get(&self, k: &ExpID) -> Option<ExpID> {
        match self {
            MatchEnv::Base() => None,
            MatchEnv::Extended(b, v) => v.into_iter().filter(|(i,_)|i==k).map(|(_,v)|v.clone()).next().or_else(||b.get(k).map(|val|val.push_local(v.len()))),
        }
    }
}

#[derive(Debug)]
pub enum Proof {
    Sequence(TruthRef, Vec<(Direction, TruthRef, Tree)>),
    Block(Vec<(String, Proof)>, Box<Proof>),
    Match(Exp, Vec<(Pattern, Proof)>),
}

impl TruthRef {
    pub fn new(name: String, gen: Vec<Type>, par: Vec<Exp>) -> Self {
        TruthRef { name, gen, par }
    }

    pub fn get(&self, env: &LocalEnvs) -> Option<ExpID> {
        let truth = env.truth.get(env.truth.get_id(&self.name)?)?;
        let gen: Vec<_> = self.gen.iter().map(|g|g.to_id(env)).collect::<Option<_>>()?;
        let par: Vec<_> = self.par.iter().map(|p|p.to_id(env)).collect::<Option<_>>()?;
        truth.get(gen, par)
    }

    pub fn apply(&self, dir: Direction, path: &Tree, exp: ExpID, env: &LocalEnvs, match_env: &MatchEnv) -> Option<ExpID> {
        if self.name == "wrap" {
            if self.par.len() != 1 { return None; }
            let par = self.par[0].to_id(env)?;

            match dir {
                Direction::Forwards => unimplemented!(),
                Direction::Backwards => {
                    exp.apply(path, 0, &|e, i|
                        if *e == par.push_local(i) {
                            match e {
                                ExpID::Var(id, _) => env.exp.get(*id)?.val().map(|e|e.push_local(i)),
                                ExpID::Call(box ExpID::Lambda(p, f), box arg) => p.match_exp(arg.clone(), env).map(|v|f.set(&v)),
                                ExpID::Call(box ExpID::Var(id, gs), box arg) => {
                                    match env.exp.get(*id)?.val()? {
                                        ExpID::Lambda(p, f) => p.match_exp(arg.clone(), env).map(|v|f.set(&v)),
                                        _ => unimplemented!(),
                                    }
                                },
                                ExpID::Match(box e, v) => v.into_iter().filter_map(|(p,a)|{let v = p.match_exp(e.clone(), env)?; Some(a.set(&v))}).next(),
                                _ => unimplemented!(),
                            }
                        } else {
                            None
                        }
                    )
                },
            }
        } else if self.name == "match" {
            if self.par.len() != 1 { return None; }
            let par = self.par[0].to_id(env)?;
            let res = match_env.get(&par)?;

            match dir {
                Direction::Forwards => {
                    exp.apply(path, 0, &|e, i|
                        if *e == par.push_local(i) {
                            Some(res.push_local(i))
                        } else {
                            None
                        }
                    )
                },
                Direction::Backwards => {
                    exp.apply(path, 0, &|e, i|
                        if *e == res.push_local(i) {
                            Some(par.push_local(i))
                        } else {
                            None
                        }
                    )
                },
            }
        } else {
            unimplemented!()
        }
    }
}

impl Proof {
    pub fn execute(&self, env: &LocalEnvs, match_env: &MatchEnv) -> Option<ExpID> {
        match self {
            Proof::Sequence(initial, rest) => {
                let mut proof = initial.get(env)?;
                for (dir, truth, path) in rest {
                    proof = truth.apply(*dir, path, proof, env, match_env)?;
                }
                Some(proof)
            },
            Proof::Match(e, v) => {
                let e = e.to_id(env)?;
                let mut re = None;
                for (pattern, proof) in v {
                    let p = proof.execute(env, &match_env.scope(expand(0, &e.push_local(pattern.bound().len()), &pattern.to_id(env)?)?))?;
                    if let Some(re) = &re {
                        if *re != p {
                            return None;
                        }
                        continue;
                    }
                    re = Some(p);
                }
                re
            }
            _ => unimplemented!(),
        }
    }
}

fn expand(i: usize, e: &ExpID, p: &PatternID) -> Option<Vec<(ExpID, ExpID)>> {
    let mut v = vec![(e.clone(), p.to_exp(i))];
    if let (ExpID::Tuple(es), PatternID::Tuple(ps)) = (e, p) {
        if es.len() != ps.len() { return None; }
        let mut i = i;
        for (e, p) in es.into_iter().zip(ps) {
            let b = p.bound().len();
            v.extend(expand(i, e, p)?);
            i += b;
        }
    }
    Some(v)
}