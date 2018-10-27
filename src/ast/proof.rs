use env::Path;
use envs::LocalNamespaces;
use super::{ Type, Pattern, Exp, ErrAst, ToID };
use id::renamed::{ TruthRefID, ProofID, Direction, RefType };
use tree::Tree;

#[derive(Debug)]
pub struct TruthRef {
    name: Path,
    gen: Vec<Type>,
    par: Option<Exp>,
}

impl ToID for TruthRef {
    type To = TruthRefID;
    fn to_id(&self, env: &LocalNamespaces) -> Result<TruthRefID, ErrAst> {
        let id = match self.name.as_ref() {
            [n] if n == "def" => RefType::Def,
            [n] if n == "match" => RefType::Match,
            _ => RefType::Ref(env.get_truth(&self.name)?),
        };
        let gen = self.gen.to_id(env)?;
        let par = self.par.to_id(env)?;
        Ok(TruthRefID::new(id, gen, par))
    }
}

impl TruthRef {
    pub fn new(name: Path, gen: Vec<Type>, par: Option<Exp>) -> Self {
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
    fn to_id(&self, env: &LocalNamespaces) -> Result<ProofID, ErrAst> {
        Ok(match self {
            Proof::Sequence(initial, rest) => ProofID::Sequence(initial.to_id(env)?, rest.into_iter().map(|(d,p,t)|Ok((*d, p.to_id(env)?, t.clone()))).collect::<Result<_,ErrAst>>()?),
            Proof::Block(vars, end) => unimplemented!(),
            Proof::Match(exp, cases) => ProofID::Match(exp.to_id(env)?, cases.to_id(env)?)
        })
    }
}
