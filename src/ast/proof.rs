use env::Path;
use envs::LocalEnvs;
use super::{ Type, Pattern, Exp, ErrAst, ToID };
use id::renamed::{ TruthRefID, ProofID, Direction, RefType };
use tree::Tree;

#[derive(Debug)]
pub struct TruthRef<T> {
    name: Path<T>,
    gen: Vec<Type<T>>,
    par: Option<Exp<T>>,
}

impl<T: Clone + AsRef<str>> ToID<T> for TruthRef<T> {
    type To = TruthRefID;
    fn to_id<'a>(&self, env: &LocalEnvs<'a>) -> Result<TruthRefID, ErrAst<T>> {
        let id = match self.name.as_ref() {
            [s] if s.as_ref() == "def" => RefType::Def,
            [s] if s.as_ref() == "match" => RefType::Match,
            _ => RefType::Ref(env.truth.get_id(&self.name).map_err(ErrAst::UnknownTruth)?),
        };
        let gen: Vec<_> = self.gen.to_id(env)?;
        let par = self.par.to_id(env)?;
        Ok(TruthRefID::new(id, gen, par))
    }
}

impl<T> TruthRef<T> {
    pub fn new(name: Path<T>, gen: Vec<Type<T>>, par: Option<Exp<T>>) -> Self {
        TruthRef { name, gen, par }
    }
}

#[derive(Debug)]
pub enum Proof<T> {
    Sequence(TruthRef<T>, Vec<(Direction, TruthRef<T>, Tree)>),
    Block(Vec<(T, Proof<T>)>, Box<Proof<T>>),
    Match(Exp<T>, Vec<(Pattern<T>, Proof<T>)>),
}

impl<T: Clone + AsRef<str>> ToID<T> for Proof<T> {
    type To = ProofID;
    fn to_id<'a>(&self, env: &LocalEnvs<'a>) -> Result<ProofID, ErrAst<T>> {
        Ok(match self {
            Proof::Sequence(initial, rest) => ProofID::Sequence(initial.to_id(env)?, rest.into_iter().map(|(d,p,t)|Ok((*d, p.to_id(env)?, t.clone()))).collect::<Result<_,ErrAst<T>>>()?),
            Proof::Block(vars, end) => unimplemented!(),
            Proof::Match(exp, cases) => ProofID::Match(exp.to_id(env)?, cases.to_id(env)?)
        })
    }
}
