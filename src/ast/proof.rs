use envs::LocalEnvs;
use ast::{ Type, Pattern, Exp };
use id::renamed::ExpID;
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

    pub fn apply(&self, dir: Direction, path: &Tree, exp: ExpID, env: &LocalEnvs) -> Option<ExpID> {
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
                                ExpID::Match(box e, v) => v.into_iter().filter_map(|(p,a)|{let v = p.match_exp(e.clone(), env)?; Some(a.set(&v))}).next(),
                                _ => unimplemented!(),
                            }
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
    pub fn execute(&self, env: &LocalEnvs) -> Option<ExpID> {
        match self {
            Proof::Sequence(initial, rest) => {
                let mut proof = initial.get(env)?;
                for (dir, truth, path) in rest {
                    proof = truth.apply(*dir, path, proof, env)?;
                }
                Some(proof)
            },
            _ => unimplemented!(),
        }
    }
}