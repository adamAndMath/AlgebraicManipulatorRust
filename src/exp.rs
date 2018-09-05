use exp_id::ExpID;
use envs::LocalEnvs;
use ty::Type;

pub enum Exp {
    Var(String),
    Tuple(Vec<Exp>),
    Lambda(Vec<(String, Type)>, Box<Exp>),
    Call(Box<Exp>, Box<Exp>),
}
