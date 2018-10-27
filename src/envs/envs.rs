use env::{ Env, LocalEnv };
use super::{ LocalEnvs, ExpVal, TypeVal, TruthVal };

#[derive(Debug)]
pub struct Envs {
    pub ty: Env<TypeVal>,
    pub exp: Env<ExpVal>,
    pub truth: Env<TruthVal>,
}

impl Envs {
    pub fn new(types: Vec<TypeVal>, exps: Vec<ExpVal>, truths: Vec<TruthVal>) -> Self {
        Envs {
            exp: Env::new(exps),
            ty: Env::new(types),
            truth: Env::new(truths),
        }
    }

    pub fn local<'b>(&'b self) -> LocalEnvs<'b> {
        LocalEnvs {
            exp: LocalEnv::new(&self.exp),
            ty: LocalEnv::new(&self.ty),
            truth: LocalEnv::new(&self.truth),
        }
    }

    #[cfg(test)]
    pub fn lens(&self) -> (usize, usize, usize) {
        (self.ty.len(), self.exp.len(), self.truth.len())
    }
}
