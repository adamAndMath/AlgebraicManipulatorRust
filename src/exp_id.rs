use id::ID;
use ty::{ Variance::*, TypeID };
use envs::LocalEnvs;

#[derive(Debug)]
pub enum ExpID {
    Var(ID),
    Tuple(Vec<ExpID>),
    Lambda(Vec<TypeID>, Box<ExpID>),
    Call(Box<ExpID>, Box<ExpID>),
}
