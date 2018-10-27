use predef::*;
use envs::*;
use variance::Variance;
use ast::{ Type, Pattern, Exp, Element, ToID, ErrAst };
use id::renamed::TypeID;


#[test]
fn struct_empty() {
    let mut space = predef_space();
    let mut env = predef();
    let lens = env.lens();

    script!{space, env,
        struct Test;
    }

    let e_id = space.get_exp(&path!(Test)).unwrap();
    let ty_id = ttype!(Test).to_id(&space.local()).unwrap();
    let mut type_val = TypeVal::new(vec!());
    type_val.push_atom(e_id);

    assert_eq!(env.exp[e_id], ExpVal::new_empty(ty_id, 0));
    assert_eq!(space.get_type(&path!(Test)).map(|id|&env.ty[id]), Ok(&type_val));

    assert_eq!(env.lens(), (lens.0+1, lens.1+1, lens.2));
}

#[test]
fn struct_tuple() {
    let mut space = predef_space();
    let mut env = predef();
    let lens = env.lens();

    script!{space, env,
        struct A;
        struct B;
        struct Test(A, B);
    }

    let e_id = space.get_exp(&path!(Test)).unwrap();
    let ty_id = ::predef::func(ttype!((A, B)).to_id(&space.local()).unwrap(), ttype!(Test).to_id(&space.local()).unwrap());
    let mut type_val = TypeVal::new(vec!());
    type_val.push_comp(e_id);

    assert_eq!(env.exp[e_id], ExpVal::new_empty(ty_id, 0));
    assert_eq!(space.get_type(&path!(Test)).map(|id|&env.ty[id]), Ok(&type_val));

    assert_eq!(env.lens(), (lens.0+3, lens.1+3, lens.2));
}

#[test]
fn enum_option() {
    let mut space = predef_space();
    let mut env = predef();
    let lens = env.lens();
    
    script!{space, env,
        enum Option[T] { Some(T), None }
    }

    assert_eq!(space.get_exp(&path!(None)), Err(ErrAst::UnknownVar(path!(None))));
    assert_eq!(space.get_exp(&path!(Some)), Err(ErrAst::UnknownVar(path!(Some))));
    let option_id = space.get_type(&path!(Option)).unwrap();
    let none_id = space.get_exp(&path!(Option::None)).unwrap();
    let some_id = space.get_exp(&path!(Option::Some)).unwrap();
    assert_eq!(env.exp[none_id].ty(&[type_id!(BOOL_ID)]), type_id!(option_id[BOOL_ID]));
    assert_eq!(env.exp[some_id].ty(&[type_id!(BOOL_ID)]), type_id!(FN_ID[BOOL_ID, option_id[BOOL_ID]]));

    assert_eq!(env.lens(), (lens.0+1, lens.1+2, lens.2));
}

#[test]
fn letting() {
    let mut space = predef_space();
    let mut env = predef();
    let lens = env.lens();

    script!{space, env,
        enum Nat { Zero, Succ(Nat) }
        let two = Nat::Succ(Nat::Succ(Nat::Zero));
        let two_marked: Nat = Nat::Succ(Nat::Succ(Nat::Zero));
    }

    assert_eq!(space.get_exp(&path!(two)).map(|id|&env.exp[id]), space.get_exp(&path!(two_marked)).map(|id|&env.exp[id]));

    assert_eq!(env.lens(), (lens.0+1, lens.1+4, lens.2));
}

#[test]
fn func() {
    let mut space = predef_space();
    let mut env = predef();
    let lens = env.lens();

    script!{space, env,
        enum Nat { Zero, Succ(Nat) }
        fn add -> Nat {
            (a: Nat, Nat::Zero) => a,
            (a: Nat, Nat::Succ(p: Nat)) => Nat::Succ(add(a, p))
        }
    }

    let nat_id = space.get_type(&path!(Nat)).expect("Nat has not been named");
    let add_id = space.get_exp(&path!(add)).expect("add has not been named");
    let add = &env.exp[add_id];
    let space = space.local();
    let exp = exp!(
        {
            (a: Nat, Nat::Zero) => a,
            (a: Nat, Nat::Succ(p: Nat)) => Nat::Succ(add(a, p))
        }
    ).to_id(&space).expect("Failed to build lambda");

    assert_eq!(add.val(add_id.into(), &[]).expect("No expresion in add"), exp);
    assert_eq!(add.ty(&[]), type_id!(FN_ID[(nat_id, nat_id), nat_id]));
    
    assert_eq!(env.lens(), (lens.0+1, lens.1+3, lens.2));
}

#[test]
fn lists() {
    let mut space = predef_space();
    let mut env = predef();
    
    script!{space, env,
        enum List[+T] { Nil, Cons(T, List[T])}
        fn prepend[T](e: T, l: List[T]) -> List[T] = List::Cons[T](e, l);
    }
}
