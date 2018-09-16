use env::LocalEnv;
use super::{ ExpVal, TypeVal };
use id::{ Type, Exp };

#[derive(Debug)]
pub struct LocalEnvs<'a> {
    pub exp: LocalEnv<'a, ExpVal>,
    pub ty: LocalEnv<'a, TypeVal>,
}

impl<'a> LocalEnvs<'a> {
    pub fn scope<'b>(&'b self, v: Vec<(String, ExpVal)>) -> LocalEnvs<'b> where 'a: 'b {
        LocalEnvs {
            exp: self.exp.scope(v),
            ty: self.ty.scope(vec![]),
        }
    }

    pub fn scope_ty<'b>(&'b self, v: Vec<(String, TypeVal)>) -> LocalEnvs<'b> where 'a: 'b {
        LocalEnvs {
            exp: self.exp.scope(vec![]),
            ty: self.ty.scope(v),
        }
    }
    
    pub fn scope_anon<'b>(&'b self, v: Vec<ExpVal>) -> LocalEnvs<'b> where 'a: 'b {
        LocalEnvs {
            exp: self.exp.scope_anon(v),
            ty: self.ty.scope_anon(vec![]),
        }
    }
    
    pub fn scope_ty_anon<'b>(&'b self, v: Vec<TypeVal>) -> LocalEnvs<'b> where 'a: 'b {
        LocalEnvs {
            exp: self.exp.scope_anon(vec![]),
            ty: self.ty.scope_anon(v),
        }
    }
}
