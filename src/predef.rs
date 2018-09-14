use std::marker::PhantomData;
use env::ID;
use envs::*;
use variance::Variance::*;
use id::Type;

pub const BOOL_ID: ID<TypeVal> = ID(0, PhantomData);
pub const FN_ID: ID<TypeVal> = ID(1, PhantomData);

pub fn predef() -> (Vec<ExpVal>, Vec<TypeVal>) {
    let mut bool_ty = TypeVal::new(vec![]);
    bool_ty.push_atom(ID::new(0));
    bool_ty.push_atom(ID::new(1));

    (
        vec![
            ExpVal::new_empty(Type::Gen(BOOL_ID.into(), vec![])),
            ExpVal::new_empty(Type::Gen(BOOL_ID.into(), vec![])),
        ],
        vec![
            bool_ty,
            TypeVal::new(vec![Contravariant, Covariant]),
        ],
    )
}

pub fn get_fn_types(ty: Type) -> Option<(Type, Type)> {
    match ty {
        Type::Gen(f, v) => {
            if f != FN_ID.into() { return None }
            match v[..] {
                [(Contravariant, ref p), (Covariant, ref b)] => Some((p.clone(), b.clone())),
                _ => None,
            }
        },
        _ => None,
    }
}