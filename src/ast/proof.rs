use env::Path;
use envs::{ NameData, Namespaces };
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
    fn to_id(&self, env: &Namespaces) -> Result<TruthRefID, ErrAst<T>> {
        let id = match self.name.as_ref() {
            [n] if n.as_ref() == "def" => RefType::Def,
            [n] if n.as_ref() == "match" => RefType::Match,
            _ => RefType::Ref(env.get_truth(&self.name)?),
        };
        let gen = self.gen.to_id(env)?;
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
    Block(T, Box<Proof<T>>, Box<Proof<T>>),
    Match(Exp<T>, Vec<(Pattern<T>, Proof<T>)>),
}

impl<T: Clone + AsRef<str>> ToID<T> for Proof<T> {
    type To = ProofID;
    fn to_id(&self, env: &Namespaces) -> Result<ProofID, ErrAst<T>> {
        Ok(match self {
            Proof::Sequence(initial, rest) => ProofID::Sequence(initial.to_id(env)?, rest.into_iter().map(|(d,p,t)|Ok((*d, p.to_id(env)?, t.clone()))).collect::<Result<_,ErrAst<T>>>()?),
            Proof::Block(n, def, proof) => {
                let mut names = NameData::new();
                let proof = proof.to_id(&env.scope_truth(&mut names, vec![n]))?;
                ProofID::Block(def.to_id(env)?, proof)
            },
            Proof::Match(exp, cases) => ProofID::Match(exp.to_id(env)?, cases.to_id(env)?),
        })
    }
}
