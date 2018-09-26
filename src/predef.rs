use std::marker::PhantomData;
use env::{ ID, LocalID };
use envs::*;
use variance::Variance::*;
use id::Type;

pub const TRUE_ID: ID<ExpVal> = ID(0, PhantomData);
pub const FALSE_ID: ID<ExpVal> = ID(1, PhantomData);
pub const EXISTS_ID: ID<ExpVal> = ID(2, PhantomData);
pub const FORALL_ID: ID<ExpVal> = ID(3, PhantomData);
pub const EQ_ID: ID<ExpVal> = ID(4, PhantomData);

pub const BOOL_ID: ID<TypeVal> = ID(0, PhantomData);
pub const FN_ID: ID<TypeVal> = ID(1, PhantomData);

pub fn predef() -> (Vec<ExpVal>, Vec<TypeVal>, Vec<TruthVal>) {
    let mut bool_ty = TypeVal::new(vec![]);
    bool_ty.push_atom(ID::new(0));
    bool_ty.push_atom(ID::new(1));

    let f_ty = Type::Gen(FN_ID.into(), vec![
        (Contravariant, Type::Gen(FN_ID.into(), vec![
            (Contravariant, Type::Gen(LocalID::new(0), vec![])),
            (Covariant, Type::Gen(BOOL_ID.into(), vec![])),
        ])),
        (Covariant, Type::Gen(BOOL_ID.into(), vec![]))
    ]);
    let eq_ty = Type::Gen(FN_ID.into(), vec![
        (Contravariant, Type::Tuple(vec![
            Type::Gen(LocalID::new(0), vec![]),
            Type::Gen(LocalID::new(0), vec![]),
        ])),
        (Covariant, Type::Gen(BOOL_ID.into(), vec![])),
    ]);

    (
        vec![
            ExpVal::new_empty(Type::Gen(BOOL_ID.into(), vec![]), 0),
            ExpVal::new_empty(Type::Gen(BOOL_ID.into(), vec![]), 0),
            ExpVal::new_empty(f_ty.clone(), 1),
            ExpVal::new_empty(f_ty.clone(), 1),
            ExpVal::new_empty(eq_ty, 1),
        ],
        vec![
            bool_ty,
            TypeVal::new(vec![Contravariant, Covariant]),
        ],
        vec![

        ],
    )
}

pub fn alias_predef(env: &mut Envs) {
    env.exp.alias("true".to_owned(), TRUE_ID);
    env.exp.alias("false".to_owned(), FALSE_ID);
    env.exp.alias("exists".to_owned(), EXISTS_ID);
    env.exp.alias("forall".to_owned(), FORALL_ID);
    env.exp.alias("eq".to_owned(), EQ_ID);

    env.ty.alias("Bool".to_owned(), BOOL_ID);
}

pub fn get_fn_types(ty: Type) -> Option<(Type, Type)> {
    match ty {
        Type::Gen(f, v) => {
            if f != FN_ID { return None }
            match v[..] {
                [(Contravariant, ref p), (Covariant, ref b)] => Some((p.clone(), b.clone())),
                _ => None,
            }
        },
        _ => None,
    }
}