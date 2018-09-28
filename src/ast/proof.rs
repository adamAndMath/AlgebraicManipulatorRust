use envs::LocalEnvs;
use super::{ Type, Pattern, Exp, ErrAst };
use id::renamed::{ ExpID, PatternID, TruthRefID, ProofID, Direction, RefType, ErrID };
use tree::Tree;

#[derive(Debug)]
pub struct TruthRef {
    name: String,
    gen: Vec<Type>,
    par: Vec<Exp>,
}

impl TruthRef {
    pub fn new(name: String, gen: Vec<Type>, par: Vec<Exp>) -> Self {
        TruthRef { name, gen, par }
    }

    pub fn to_id(&self, env: &LocalEnvs) -> Result<TruthRefID, ErrAst> {
        let id = match self.name.as_ref() {
            "wrap" => RefType::Wrap,
            "match" => RefType::Match,
            name => RefType::Ref(env.truth.get_id(name).map_err(ErrAst::UnknownTruth)?),
        };
        let gen: Vec<_> = self.gen.iter().map(|g|g.to_id(env)).collect::<Result<_,_>>()?;
        let par: Vec<_> = self.par.iter().map(|p|p.to_id(env)).collect::<Result<_,_>>()?;
        Ok(TruthRefID::new(id, gen, par))
    }
}

#[derive(Debug)]
pub enum Proof {
    Sequence(TruthRef, Vec<(Direction, TruthRef, Tree)>),
    Block(Vec<(String, Proof)>, Box<Proof>),
    Match(Exp, Vec<(Pattern, Proof)>),
}

impl Proof {
    pub fn to_id(&self, env: &LocalEnvs) -> Result<ProofID, ErrAst> {
        Ok(match self {
            Proof::Sequence(initial, rest) => ProofID::Sequence(initial.to_id(env)?, rest.into_iter().map(|(d,p,t)|Ok((*d, p.to_id(env)?, t.clone()))).collect::<Result<_,ErrAst>>()?),
            Proof::Block(vars, end) => unimplemented!(),
            Proof::Match(exp, cases) => ProofID::Match(exp.to_id(env)?, cases.into_iter().map(|(p,t)|Ok((p.to_id(env)?, t.to_id(env)?))).collect::<Result<_,ErrAst>>()?)
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