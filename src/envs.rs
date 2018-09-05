use env::Env;
use local_env::LocalEnv;
use exp_id::ExpID;
use ty::{ Variance, TypeID };

#[derive(Debug)]
pub struct Envs<'a> {
    exp: Env<'a, (Option<ExpID>, TypeID)>,
    ty: Env<'a, (Vec<Variance>, Vec<usize>, Vec<usize>)>,
}


#[derive(Debug)]
pub struct LocalEnvs<'a> {
    pub exp: LocalEnv<'a, (Option<ExpID>, TypeID)>,
    pub ty: &'a Env<'a, (Vec<Variance>, Vec<usize>, Vec<usize>)>,
}

impl<'a> LocalEnvs<'a> {
    pub fn scope<'b>(&'b self, v: Vec<(String, (Option<ExpID>, TypeID))>) -> LocalEnvs<'b> where 'a: 'b {
        LocalEnvs {
            exp: self.exp.scope(v),
            ty: self.ty,
        }
    }
    
    pub fn scope_anon<'b>(&'b self, v: Vec<(Option<ExpID>, TypeID)>) -> LocalEnvs<'b> where 'a: 'b {
        LocalEnvs {
            exp: self.exp.scope_anon(v),
            ty: self.ty,
        }
    }
}