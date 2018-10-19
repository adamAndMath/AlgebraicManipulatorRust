use env::LocalEnv;
use super::{ ExpVal, TypeVal, TruthVal };

#[derive(Debug)]
pub struct LocalEnvs<'a> {
    pub exp: LocalEnv<'a, ExpVal>,
    pub ty: LocalEnv<'a, TypeVal>,
    pub truth: LocalEnv<'a, TruthVal>,
}

impl<'a> LocalEnvs<'a> {
    pub fn scope<'b, S: AsRef<str>>(&'b self, v: Vec<(S, ExpVal)>) -> LocalEnvs<'b> where 'a: 'b {
        LocalEnvs {
            exp: self.exp.scope(v),
            ty: self.ty.scope::<S>(vec![]),
            truth: self.truth.scope::<S>(vec![]),
        }
    }

    pub fn scope_ty<'b, S: AsRef<str>>(&'b self, v: Vec<(S, TypeVal)>) -> LocalEnvs<'b> where 'a: 'b {
        LocalEnvs {
            exp: self.exp.scope::<S>(vec![]),
            ty: self.ty.scope(v),
            truth: self.truth.scope::<S>(vec![]),
        }
    }
    
    pub fn scope_truth<'b, S: AsRef<str>>(&'b self, v: Vec<(S, TruthVal)>) -> LocalEnvs<'b> where 'a: 'b {
        LocalEnvs {
            exp: self.exp.scope::<S>(vec![]),
            ty: self.ty.scope::<S>(vec![]),
            truth: self.truth.scope(v),
        }
    }
}
