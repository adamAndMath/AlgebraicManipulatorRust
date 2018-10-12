use predef::*;
use env::{ Env, LocalEnv };
use super::{ LocalEnvs, ExpVal, TypeVal, TruthVal };

#[derive(Debug)]
pub struct Envs<'a> {
    pub exp: Env<'a, ExpVal>,
    pub ty: Env<'a, TypeVal>,
    pub truth: Env<'a, TruthVal>,
}

impl<'a> Envs<'a> {
    pub fn new(exps: &'a mut Vec<ExpVal>, tys: &'a mut Vec<TypeVal>, truths: &'a mut Vec<TruthVal>) -> Self {
        Envs {
            exp: Env::new(exps),
            ty: Env::new(tys),
            truth: Env::new(truths),
        }
    }

    pub fn child_scope<E, F: Fn(&mut Envs) -> Result<(), E>>(&mut self, n: String, f: F) -> Result<(), E> {
        let (exp, ty, truth) = {
            let mut child = Envs {
                exp: self.exp.child_scope(),
                ty: self.ty.child_scope(),
                truth: self.truth.child_scope(),
            };
            alias_predef(&mut child);
            f(&mut child)?;
            (child.exp.to_val(), child.ty.to_val(), child.truth.to_val())
        };
        self.exp.add_val(n.clone(), exp);
        self.ty.add_val(n.clone(), ty);
        self.truth.add_val(n.clone(), truth);
        Ok(())
    }

    pub fn local<'b>(&'b self) -> LocalEnvs<'b> where 'a: 'b {
        LocalEnvs {
            exp: LocalEnv::new(&self.exp),
            ty: LocalEnv::new(&self.ty),
            truth: LocalEnv::new(&self.truth),
        }
    }
}
