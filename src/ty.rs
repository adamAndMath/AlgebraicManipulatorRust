use envs::{ TypeVal, LocalEnvs };
use variance::Variance;
use env::local_id::LocalID;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeID {
    Gen(LocalID<TypeVal>, Vec<(Variance, TypeID)>),
    Tuple(Vec<TypeID>),
}
