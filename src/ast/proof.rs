use envs::LocalEnvs;
use super::{ Type, Pattern, Exp, ErrAst, ToID };
use id::renamed::{ TruthRefID, ProofID, Direction, RefType, ErrID };
use tree::Tree;

#[derive(Debug)]
pub struct TruthRef {
    name: String,
    gen: Vec<Type>,
    par: Vec<Exp>,
}

impl ToID for TruthRef {
    type To = TruthRefID;
    fn to_id(&self, env: &LocalEnvs) -> Result<TruthRefID, ErrAst> {
        let id = match self.name.as_ref() {
            "wrap" => RefType::Wrap,
            "match" => RefType::Match,
            name => RefType::Ref(env.truth.get_id(name).map_err(ErrAst::UnknownTruth)?),
        };
        let gen: Vec<_> = self.gen.to_id(env)?;
        let par: Vec<_> = self.par.to_id(env)?;
        Ok(TruthRefID::new(id, gen, par))
    }
}

impl TruthRef {
    pub fn new(name: String, gen: Vec<Type>, par: Vec<Exp>) -> Self {
        TruthRef { name, gen, par }
    }
}

#[derive(Debug)]
pub enum Proof {
    Sequence(TruthRef, Vec<(Direction, TruthRef, Tree)>),
    Block(Vec<(String, Proof)>, Box<Proof>),
    Match(Exp, Vec<(Pattern, Proof)>),
}

impl ToID for Proof {
    type To = ProofID;
    fn to_id(&self, env: &LocalEnvs) -> Result<ProofID, ErrAst> {
        Ok(match self {
            Proof::Sequence(initial, rest) => ProofID::Sequence(initial.to_id(env)?, rest.into_iter().map(|(d,p,t)|Ok((*d, p.to_id(env)?, t.clone()))).collect::<Result<_,ErrAst>>()?),
            Proof::Block(vars, end) => unimplemented!(),
            Proof::Match(exp, cases) => ProofID::Match(exp.to_id(env)?, cases.to_id(env)?)
        })
    }
}
