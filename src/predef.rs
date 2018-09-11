use std::marker::PhantomData;
use envs::*;
use id::ID;
use variance::Variance::*;
use ty::TypeID;

pub const BOOL_ID: ID<TypeVal> = ID(0, PhantomData);
pub const FN_ID: ID<TypeVal> = ID(1, PhantomData);

pub fn predef() -> (Vec<ExpVal>, Vec<TypeVal>) {
    let mut bool_ty = TypeVal::new(vec![]);
    bool_ty.push_atom(ID::new(0));
    bool_ty.push_atom(ID::new(1));

    (
        vec![
            ExpVal::new_empty(TypeID::Gen(BOOL_ID.into(), vec![])),
            ExpVal::new_empty(TypeID::Gen(BOOL_ID.into(), vec![])),
        ],
        vec![
            bool_ty,
            TypeVal::new(vec![Contravariant, Covariant]),
        ],
    )
}