use env::{ Env, LocalEnv };
use super::{ EnvsData, LocalEnvs, ExpVal, TypeVal, TruthVal };

#[derive(Debug)]
pub struct Envs<'a> {
    pub exp: Env<'a, ExpVal>,
    pub ty: Env<'a, TypeVal>,
    pub truth: Env<'a, TruthVal>,
}

impl<'a> Envs<'a> {
    pub fn new(data: &'a mut EnvsData) -> Self {
        Envs {
            exp: Env::new(&mut data.exps),
            ty: Env::new(&mut data.types),
            truth: Env::new(&mut data.truths),
        }
    }

    pub fn child_scope<E, F: Fn(&mut Envs) -> Result<(), E>>(&mut self, n: String, f: F) -> Result<(), E> {
        let (exp, ty, truth) = {
            let mut child = Envs {
                exp: self.exp.child_scope(),
                ty: self.ty.child_scope(),
                truth: self.truth.child_scope(),
            };
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
