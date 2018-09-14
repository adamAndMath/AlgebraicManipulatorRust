use envs::TypeVal;
use variance::Variance;
use env::LocalID;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeID {
    Gen(LocalID<TypeVal>, Vec<(Variance, TypeID)>),
    Tuple(Vec<TypeID>),
}
