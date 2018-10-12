use parser::Parse;
use predef::*;
use env::Path;
use envs::*;
use ast::{ Type, Exp, Element, ToID };
use id::renamed::TypeID;


#[test]
fn struct_empty() {
    let (mut exps, mut tys, mut truths) = predef();
    let lens = (exps.len(), tys.len(), truths.len());
    {
        let mut env = Envs::new(&mut exps, &mut tys, &mut truths);
        Element::parse("struct Test").define(&mut env).unwrap();

        let e_id = env.exp.get_id(&Path::parse("Test")).unwrap();
        let ty_id = Type::parse("Test").to_id(&env.local()).unwrap();
        let mut type_val = TypeVal::new(vec!());
        type_val.push_atom(e_id);

        assert_eq!(env.exp.get(e_id), Ok(&ExpVal::new_empty(ty_id, 0)));
        assert_eq!(env.ty.get_id(&Path::parse("Test")).map(|id|env.ty.get(id)), Ok(Ok(&type_val)))
    }

    assert_eq!((exps.len(), tys.len(), truths.len()), (lens.0+1, lens.1+1, lens.2));
}

#[test]
fn struct_tuple() {
    let (mut exps, mut tys, mut truths) = predef();
    let lens = (exps.len(), tys.len(), truths.len());
    {
        let mut env = Envs::new(&mut exps, &mut tys, &mut truths);
        env.ty.alias("fn".to_owned(), FN_ID.into());
        Element::parse("struct A").define(&mut env).unwrap();
        Element::parse("struct B").define(&mut env).unwrap();
        Element::parse("struct Test(A, B)").define(&mut env).unwrap();

        let e_id = env.exp.get_id(&Path::parse("Test")).unwrap();
        let ty_id = Type::parse("fn<(A, B), Test>").to_id(&env.local()).unwrap();
        let mut type_val = TypeVal::new(vec!());
        type_val.push_comp(e_id);

        assert_eq!(env.exp.get(e_id), Ok(&ExpVal::new_empty(ty_id, 0)));
        assert_eq!(env.ty.get_id(&Path::parse("Test")).map(|id|env.ty.get(id)), Ok(Ok(&type_val)))
    }

    assert_eq!((exps.len(), tys.len(), truths.len()), (lens.0+3, lens.1+3, lens.2));
}

#[test]
fn enum_option() {
    let (mut exps, mut tys, mut truths) = predef();
    let lens = (exps.len(), tys.len(), truths.len());
    {
        let mut env = Envs::new(&mut exps, &mut tys, &mut truths);
        alias_predef(&mut env);
        env.ty.alias("fn".to_owned(), FN_ID.into());
        Element::parse("enum Option<T> { Some(T), None }").define(&mut env).unwrap();

        assert_eq!(env.exp.get_id(&Path::parse("None")), Err(Path::parse("None")));
        assert_eq!(env.exp.get_id(&Path::parse("Some")), Err(Path::parse("Some")));
        let none_id = env.exp.get_id(&Path::parse("Option::None")).unwrap();
        let some_id = env.exp.get_id(&Path::parse("Option::Some")).unwrap();
        assert_eq!(env.exp.get(none_id).map(|e|e.ty(&[type_id!(BOOL_ID)])), Ok(Type::parse("Option<Bool>").to_id(&env.local()).unwrap()));
        assert_eq!(env.exp.get(some_id).map(|e|e.ty(&[type_id!(BOOL_ID)])), Ok(Type::parse("fn<Bool, Option<Bool>>").to_id(&env.local()).unwrap()));
    }

    assert_eq!((exps.len(), tys.len(), truths.len()), (lens.0+2, lens.1+1, lens.2));
}

#[test]
fn letting() {
    let (mut exps, mut tys, mut truths) = predef();
    let lens = (exps.len(), tys.len(), truths.len());
    {
        let mut env = Envs::new(&mut exps, &mut tys, &mut truths);
        Element::parse("enum Nat { Zero, Succ(Nat) }").define(&mut env).unwrap();
        Element::parse("let two = Nat::Succ(Nat::Succ(Nat::Zero))").define(&mut env).unwrap();
        Element::parse("let two_marked: Nat = Nat::Succ(Nat::Succ(Nat::Zero))").define(&mut env).unwrap();

        assert_eq!(env.exp.get_id(&Path::parse("two")).map(|id|env.exp.get(id).unwrap()), env.exp.get_id(&Path::parse("two_marked")).map(|id|env.exp.get(id).unwrap()));
    }

    assert_eq!((exps.len(), tys.len(), truths.len()), (lens.0+4, lens.1+1, lens.2));
}

#[test]
fn func() {
    let (mut exps, mut tys, mut truths) = predef();
    let lens = (exps.len(), tys.len(), truths.len());
    {
        let mut env = Envs::new(&mut exps, &mut tys, &mut truths);
        env.ty.alias("fn".to_owned(), FN_ID.into());
        Element::parse("enum Nat { Zero, Succ(Nat) }").define(&mut env).expect("Failed to define Nat");
        Element::parse(
            "fn add -> Nat {
                (a: Nat, Nat::Zero) => a,
                (a: Nat, Nat::Succ(p: Nat)) => Nat::Succ(add(a, p))
            }"
        ).define(&mut env).expect("Failed to define add");

        let env = env.local();
        let add_id = env.exp.get_id(&Path::parse("add")).expect("add has not been named");
        let add = env.exp.get(add_id).expect("add has not been added to the environment");
        let exp = Exp::parse(
            "{
                (a: Nat, Nat::Zero) => a,
                (a: Nat, Nat::Succ(p: Nat)) => Nat::Succ(add(a, p))
            }"
        ).to_id(&env).expect("Failed to build lambda");

        assert_eq!(add.val(add_id, &[]).expect("No expresion in add"), exp);
        assert_eq!(add.ty(&[]), Type::parse("fn<(Nat, Nat), Nat>").to_id(&env).expect("Failed to find type (Nat, Nat) -> Nat"));
    }
    
    assert_eq!((exps.len(), tys.len(), truths.len()), (lens.0+3, lens.1+1, lens.2));
}

#[test]
fn lists() {
    let (mut exps, mut tys, mut truths) = predef();
    let mut env = Envs::new(&mut exps, &mut tys, &mut truths);
    
    Element::parse("enum List<+T> { Nil, Cons(T, List<T>)}").define(&mut env).unwrap();
    Element::parse("fn prepend<T>(e: T, l: List<T>) -> List<T> = List::Cons<T>(e, l)").define(&mut env).unwrap();
}
