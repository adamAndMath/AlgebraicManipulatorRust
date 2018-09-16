use predef::*;
use envs::*;
use variance::Variance;
use ast::{ Type, Pattern, Exp, Element};


#[test]
fn struct_empty() {
    let (mut exps, mut tys, mut truths) = predef();
    let lens = (exps.len(), tys.len(), truths.len());
    {
        let mut env = Envs::new(&mut exps, &mut tys, &mut truths);
        element!(struct Test).define(&mut env).unwrap();

        let e_id = env.exp.get_id("Test").unwrap();
        let ty_id = ttype!(Test).to_id(&env.local()).unwrap();
        let mut type_val = TypeVal::new(vec!());
        type_val.push_atom(e_id);

        assert_eq!(env.exp.get(e_id), Some(&ExpVal::new_empty(ty_id, 0)));
        assert_eq!(env.ty.get_id("Test").and_then(|id|env.ty.get(id)), Some(&type_val))
    }

    assert_eq!((exps.len(), tys.len(), truths.len()), (lens.0+1, lens.1+1, lens.2));
}

#[test]
fn struct_tuple() {
    let (mut exps, mut tys, mut truths) = predef();
    let lens = (exps.len(), tys.len(), truths.len());
    {
        let mut env = Envs::new(&mut exps, &mut tys, &mut truths);
        env.ty.alias("fn".to_owned(), FN_ID);
        element!(struct A).define(&mut env).unwrap();
        element!(struct B).define(&mut env).unwrap();
        element!(struct Test(A, B)).define(&mut env).unwrap();

        let e_id = env.exp.get_id("Test").unwrap();
        let ty_id = ttype!(fn[(A, B), Test]).to_id(&env.local()).unwrap();
        let mut type_val = TypeVal::new(vec!());
        type_val.push_comp(e_id);

        assert_eq!(env.exp.get(e_id), Some(&ExpVal::new_empty(ty_id, 0)));
        assert_eq!(env.ty.get_id("Test").and_then(|id|env.ty.get(id)), Some(&type_val))
    }

    assert_eq!((exps.len(), tys.len(), truths.len()), (lens.0+3, lens.1+3, lens.2));
}

#[test]
fn letting() {
    let (mut exps, mut tys, mut truths) = predef();
    let lens = (exps.len(), tys.len(), truths.len());
    {
        let mut env = Envs::new(&mut exps, &mut tys, &mut truths);
        element!(enum Nat { Zero, Succ(Nat) }).define(&mut env).unwrap();
        element!(let two = Succ(Succ(Zero))).define(&mut env).unwrap();
        element!(let two_marked: Nat = Succ(Succ(Zero))).define(&mut env).unwrap();

        assert_eq!(env.exp.get_id("two").and_then(|id|env.exp.get(id)), env.exp.get_id("two_marked").and_then(|id|env.exp.get(id)));
    }

    assert_eq!((exps.len(), tys.len(), truths.len()), (lens.0+4, lens.1+1, lens.2));
}

#[test]
fn func() {
    let (mut exps, mut tys, mut truths) = predef();
    let lens = (exps.len(), tys.len(), truths.len());
    {
        let mut env = Envs::new(&mut exps, &mut tys, &mut truths);
        env.ty.alias("fn".to_owned(), FN_ID);
        element!(enum Nat { Zero, Succ(Nat) }).define(&mut env).expect("Failed to define Nat");
        element!(
            fn add(a: Nat, b: Nat) -> Nat = match(b, {
                Zero => a,
                Succ(p: Nat) => Succ(add(a, p))
            })
        ).define(&mut env).expect("Failed to define add");

        let env = env.local();
        let add_id = env.exp.get_id("add").expect("add has not been named");
        let add = env.exp.get(add_id).expect("add has not been added to the environment");
        let exp = exp!(
            (a: Nat, b: Nat) -> match(b, {
                Zero => a,
                Succ(p: Nat) => Succ(add(a, p))
            })
        ).to_id(&env).expect("Failed to build lambda");

        assert_eq!(add.val().expect("No expresion in add"), exp);
        assert_eq!(add.ty(), ttype!(fn[(Nat, Nat), Nat]).to_id(&env).expect("Failed to find type (Nat, Nat) -> Nat"));
    }
    
    assert_eq!((exps.len(), tys.len(), truths.len()), (lens.0+3, lens.1+1, lens.2));
}

#[test]
fn lists() {
    let (mut exps, mut tys, mut truths) = predef();
    let mut env = Envs::new(&mut exps, &mut tys, &mut truths);
    
    element!(enum List[+T] { Nil, Cons(T, List[T])}).define(&mut env).unwrap();
    element!(fn prepend[T](e: T, l: List[T]) -> List[T] = Cons[T](e, l)).define(&mut env).unwrap();
}
