use envs::LocalEnvs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variance {
    Invariant,
    Covariant,
    Contravariant,
}

#[derive(Debug)]
pub enum Type {
    Gen(String, Vec<Type>),
    Tuple(Vec<Type>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeID {
    Gen(usize, Vec<(Variance, TypeID)>),
    Tuple(Vec<TypeID>),
}
