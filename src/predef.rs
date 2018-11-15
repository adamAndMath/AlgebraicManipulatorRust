use std::marker::PhantomData;
use env::{ ID, PushID };
use envs::*;
use variance::Variance::*;
use id::{ Type, Pattern, Patterned, Exp, ErrID };

pub const TRUE_ID: ID<ExpVal> = ID::Predef(0, PhantomData);
pub const FALSE_ID: ID<ExpVal> = ID::Predef(1, PhantomData);
pub const EXISTS_ID: ID<ExpVal> = ID::Predef(2, PhantomData);
pub const FORALL_ID: ID<ExpVal> = ID::Predef(3, PhantomData);
pub const EQ_ID: ID<ExpVal> = ID::Predef(4, PhantomData);

pub const BOOL_ID: ID<TypeVal> = ID::Predef(0, PhantomData);
pub const FN_ID: ID<TypeVal> = ID::Predef(1, PhantomData);

pub const ID_ID: ID<TruthVal> = ID::Predef(0, PhantomData);

pub fn predef_space(data: &mut NameData) -> Namespaces {
    Namespaces::new(
        data,
        vec![
            ("Bool", BOOL_ID),
        ],
        vec![
            ("true", TRUE_ID),
            ("false", FALSE_ID),
            ("exists", EXISTS_ID),
            ("forall", FORALL_ID),
            ("eq", EQ_ID),
        ],
        vec![
            ("ID", ID_ID),
        ]
    )
}

pub fn predef_data<'a>() -> EnvData {
    let mut bool_ty = TypeVal::new(vec![]);
    bool_ty.push_atom(TRUE_ID);
    bool_ty.push_atom(FALSE_ID);

    let f_ty = func(func(Type::Gen(ID::new(0), vec![]), Type::Gen(BOOL_ID, vec![])), Type::Gen(BOOL_ID, vec![]));
    let eq_ty = func(func(Type::Gen(ID::new(0), vec![]), Type::Gen(ID::new(0), vec![])), Type::Gen(BOOL_ID, vec![]));

    EnvData {
        types: vec![
            bool_ty,
            TypeVal::new(vec![Contravariant, Covariant]),
        ],
        exps: vec![
            ExpVal::new_empty(Type::Gen(BOOL_ID, vec![]), 0),
            ExpVal::new_empty(Type::Gen(BOOL_ID, vec![]), 0),
            ExpVal::new_empty(f_ty.clone(), 1),
            ExpVal::new_empty(f_ty.clone(), 1),
            ExpVal::new_empty(eq_ty, 1),
        ],
        truths: vec![
            TruthVal::new(Exp::Call(Box::new(Exp::Var(FORALL_ID, vec![Type::Gen(ID::new(0), vec![])])), Box::new(Exp::Closure(vec![Patterned(Pattern::Var(Type::Gen(ID::new(0).push_id(1), vec![])), Exp::Call(Box::new(Exp::Var(EQ_ID, vec![Type::Gen(ID::new(0).push_id(1), vec![])])), Box::new(Exp::Tuple(vec![Exp::Var(ID::new(0), vec![]), Exp::Var(ID::new(0), vec![])]))))]))), 1)
        ],
    }
}

pub fn get_fn_types(ty: Type) -> Result<(Type, Type), ErrID> {
    if let Type::Gen(ID::Predef(1,_), v) = ty {
        if let [ref p, ref b] = v[..] {
            Ok((p.clone(), b.clone()))
        } else {
            Err(ErrID::GenericAmount(v.len(), 2))
        }
    } else {
        Err(ErrID::TypeMismatch(ty, Type::Gen(FN_ID, vec![])))
    }
}

pub fn func(input: Type, output: Type) -> Type {
    Type::Gen(FN_ID, vec![input, output])
}