use std::marker::PhantomData;
use env::{ ID, LocalID };
use envs::*;
use variance::Variance::*;
use id::{ Type, Pattern, Exp, ErrID };

pub const TRUE_ID: ID<ExpVal> = ID::Predef(0, PhantomData);
pub const FALSE_ID: ID<ExpVal> = ID::Predef(1, PhantomData);
pub const EXISTS_ID: ID<ExpVal> = ID::Predef(2, PhantomData);
pub const FORALL_ID: ID<ExpVal> = ID::Predef(3, PhantomData);
pub const EQ_ID: ID<ExpVal> = ID::Predef(4, PhantomData);

pub const BOOL_ID: ID<TypeVal> = ID::Predef(0, PhantomData);
pub const FN_ID: ID<TypeVal> = ID::Predef(1, PhantomData);

pub const ID_ID: ID<TruthVal> = ID::Predef(0, PhantomData);

pub fn predef_space() -> Namespaces {
    Namespaces::new(
        vec![
            ("Bool".to_owned(), BOOL_ID.into()),
        ],
        vec![
            ("true".to_owned(), TRUE_ID.into()),
            ("false".to_owned(), FALSE_ID.into()),
            ("exists".to_owned(), EXISTS_ID.into()),
            ("forall".to_owned(), FORALL_ID.into()),
            ("eq".to_owned(), EQ_ID.into()),
        ],
        vec![
            ("ID".to_owned(), ID_ID.into()),
        ]
    )
}

pub fn predef() -> Envs {
    let mut bool_ty = TypeVal::new(vec![]);
    bool_ty.push_atom(TRUE_ID);
    bool_ty.push_atom(FALSE_ID);

    let f_ty = func(func(Type::Gen(LocalID::new(0), vec![]), Type::Gen(BOOL_ID.into(), vec![])), Type::Gen(BOOL_ID.into(), vec![]));
    let eq_ty = func(func(Type::Gen(LocalID::new(0), vec![]), Type::Gen(LocalID::new(0), vec![])), Type::Gen(BOOL_ID.into(), vec![]));

    Envs::new(
        vec![
            bool_ty,
            TypeVal::new(vec![Contravariant, Covariant]),
        ],
        vec![
            ExpVal::new_empty(Type::Gen(BOOL_ID.into(), vec![]), 0),
            ExpVal::new_empty(Type::Gen(BOOL_ID.into(), vec![]), 0),
            ExpVal::new_empty(f_ty.clone(), 1),
            ExpVal::new_empty(f_ty.clone(), 1),
            ExpVal::new_empty(eq_ty, 1),
        ],
        vec![
            TruthVal::new(Exp::Call(Box::new(Exp::Var(FORALL_ID.into(), vec![Type::Gen(LocalID::new(0), vec![])])), Box::new(Exp::Closure(vec![(Pattern::Var(Type::Gen(LocalID::new(0), vec![])), Exp::Call(Box::new(Exp::Var(EQ_ID.into(), vec![Type::Gen(LocalID::new(0), vec![])])), Box::new(Exp::Tuple(vec![Exp::Var(LocalID::new(0), vec![]), Exp::Var(LocalID::new(0), vec![])]))))]))), 1)
        ],
    )
}

pub fn get_fn_types(ty: Type) -> Result<(Type, Type), ErrID> {
    if let Type::Gen(LocalID::Global(ID::Predef(1,_)), v) = ty {
        if let [ref p, ref b] = v[..] {
            Ok((p.clone(), b.clone()))
        } else {
            Err(ErrID::GenericAmount(v.len(), 2))
        }
    } else {
        Err(ErrID::TypeMismatch(ty, Type::Gen(FN_ID.into(), vec![])))
    }
}

pub fn func(input: Type, output: Type) -> Type {
    Type::Gen(FN_ID.into(), vec![input, output])
}