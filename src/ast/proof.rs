use envs::LocalEnvs;
use super::{ Type, Pattern, Exp, ErrAst };
use id::renamed::{ ExpID, PatternID, ErrID };
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

    pub fn get(&self, env: &LocalEnvs) -> Result<ExpID, ErrAst> {
        let truth = env.truth.get(env.truth.get_id(&self.name).map_err(ErrAst::UnknownTruth)?)?;
        let gen: Vec<_> = self.gen.iter().map(|g|g.to_id(env)).collect::<Result<_,_>>()?;
        let par: Vec<_> = self.par.iter().map(|p|p.to_id(env)).collect::<Result<_,_>>()?;
        truth.get(gen, par.clone()).ok_or(ErrAst::ErrID(ErrID::InvalidArguments(par)))
    }

    pub fn apply(&self, dir: Direction, path: &Tree, exp: ExpID, env: &LocalEnvs, match_env: &MatchEnv) -> Result<ExpID, ErrAst> {
        if self.name == "wrap" {
            if self.par.len() != 1 { return Err(ErrAst::ArgumentCount("wrap".to_owned(), 1)); }
            let par = self.par[0].to_id(env)?;

            match dir {
                Direction::Forwards => unimplemented!(),
                Direction::Backwards => {
                    exp.apply(path, 0, &|e, i| {
                        let par = par.push_local(i);
                        if *e == par {
                            match e {
                                ExpID::Var(id, _) => env.exp.get(*id)?.val().ok_or(ErrID::VarNotSet(*id)).map(|e|e.push_local(i)),
                                ExpID::Call(box ExpID::Lambda(p, f), box arg) => p.match_exp(arg.clone(), env).map(|v|f.set(&v)),
                                ExpID::Call(box ExpID::Var(id, gs), box arg) => {
                                    match env.exp.get(*id)?.val().ok_or(ErrID::VarNotSet(*id))? {
                                        ExpID::Lambda(p, f) => p.match_exp(arg.clone(), env).map(|v|f.set(&v)),
                                        _ => unimplemented!(),
                                    }
                                },
                                ExpID::Match(box e, v) =>
                                    v.into_iter()
                                        .filter_map(|(p,a)|{let v = p.match_exp(e.clone(), env).ok()?; Some(a.set(&v))})
                                        .next()
                                        .ok_or(ErrID::NoMatch(e.clone())),
                                _ => unimplemented!(),
                            }
                        } else {
                            Err(ErrID::ExpMismatch(e.clone(), par))
                        }
                    }).map_err(|e|match e {Ok(e) => e.into(), Err(e) => e.into()})
                },
            }
        } else if self.name == "match" {
            if self.par.len() != 1 { return Err(ErrAst::ArgumentCount("match".to_owned(), 1)); }
            let par = self.par[0].to_id(env)?;
            let res = match_env.get(&par).ok_or(ErrAst::ErrID(ErrID::NoMatch(par.clone())))?;

            match dir {
                Direction::Forwards => {
                    exp.apply(path, 0, &|e, i| {
                        let par = par.push_local(i);
                        if *e == par {
                            Ok(res.push_local(i))
                        } else {
                            Err(ErrAst::ErrID(ErrID::ExpMismatch(e.clone(), par)))
                        }
                    }).map_err(|e|match e {Ok(e) => e, Err(e) => e.into()})
                },
                Direction::Backwards => {
                    exp.apply(path, 0, &|e, i| {
                        let res = res.push_local(i);
                        if *e == res {
                            Ok(par.push_local(i))
                        } else {
                            Err(ErrAst::ErrID(ErrID::ExpMismatch(e.clone(), res)))
                        }
                    }).map_err(|e|match e {Ok(e) => e, Err(e) => e.into()})
                },
            }
        } else {
            unimplemented!()
        }
    }
}

impl Proof {
    pub fn execute(&self, env: &LocalEnvs, match_env: &MatchEnv) -> Result<ExpID, ErrAst> {
        Ok(match self {
            Proof::Sequence(initial, rest) => {
                let mut proof = initial.get(env)?;
                for (dir, truth, path) in rest {
                    proof = truth.apply(*dir, path, proof, env, match_env)?;
                }
                proof
            },
            Proof::Match(e, v) => {
                let e = e.to_id(env)?;
                let mut re: Option<ExpID> = None;
                for (pattern, proof) in v {
                    let p = proof.execute(env, &match_env.scope(expand(0, &e.push_local(pattern.bound().len()), &pattern.to_id(env)?)?))?;
                    if let Some(re) = &re {
                        if *re != p {
                            return Err(ErrAst::ErrID(ErrID::ExpMismatch(p, re.clone())));
                        }
                        continue;
                    }
                    re = Some(p);
                }
                re.unwrap()
            }
            _ => unimplemented!(),
        })
    }
}

fn expand(i: usize, e: &ExpID, p: &PatternID) -> Result<Vec<(ExpID, ExpID)>, ErrAst> {
    let mut v = vec![(e.clone(), p.to_exp(i))];
    if let (ExpID::Tuple(es), PatternID::Tuple(ps)) = (e, p) {
        if es.len() != ps.len() { unreachable!("This should be caught by type checker"); }
        let mut i = i;
        for (e, p) in es.into_iter().zip(ps) {
            let b = p.bound().len();
            v.extend(expand(i, e, p)?);
            i += b;
        }
    }
    Ok(v)
}