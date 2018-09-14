use envs::TypeVal;
use variance::Variance;
use env::LocalID;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Gen(LocalID<TypeVal>, Vec<(Variance, Type)>),
    Tuple(Vec<Type>),
}
