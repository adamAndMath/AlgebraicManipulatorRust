use env::Path;
use envs::LocalEnvs;
use super::{ Type, Pattern, Exp, ErrAst, ToID };
use id::renamed::{ TruthRefID, ProofID, Direction, RefType };
use tree::Tree;

#[derive(Debug)]
pub struct TruthRef<'f> {
    name: Path<'f>,
    gen: Vec<Type<'f>>,
    par: Option<Exp<'f>>,
}

impl<'f> ToID<'f> for TruthRef<'f> {
    type To = TruthRefID;
    fn to_id<'a>(&self, env: &LocalEnvs<'f, 'a>) -> Result<TruthRefID, ErrAst<'f>> {
        let id = match self.name.as_ref() {
            ["def"] => RefType::Def,
            ["match"] => RefType::Match,
            _ => RefType::Ref(env.truth.get_id(&self.name).map_err(ErrAst::UnknownTruth)?),
        };
        let gen: Vec<_> = self.gen.to_id(env)?;
        let par = self.par.to_id(env)?;
        Ok(TruthRefID::new(id, gen, par))
    }
}

impl<'f> TruthRef<'f> {
    pub fn new(name: Path<'f>, gen: Vec<Type<'f>>, par: Option<Exp<'f>>) -> Self {
        TruthRef { name, gen, par }
    }
}

#[derive(Debug)]
pub enum Proof<'f> {
    Sequence(TruthRef<'f>, Vec<(Direction, TruthRef<'f>, Tree)>),
    Block(Vec<(&'f str, Proof<'f>)>, Box<Proof<'f>>),
    Match(Exp<'f>, Vec<(Pattern<'f>, Proof<'f>)>),
}

impl<'f> ToID<'f> for Proof<'f> {
    type To = ProofID;
    fn to_id<'a>(&self, env: &LocalEnvs<'f, 'a>) -> Result<ProofID, ErrAst<'f>> {
        Ok(match self {
            Proof::Sequence(initial, rest) => ProofID::Sequence(initial.to_id(env)?, rest.into_iter().map(|(d,p,t)|Ok((*d, p.to_id(env)?, t.clone()))).collect::<Result<_,ErrAst>>()?),
            Proof::Block(vars, end) => unimplemented!(),
            Proof::Match(exp, cases) => ProofID::Match(exp.to_id(env)?, cases.to_id(env)?)
        })
    }
}
