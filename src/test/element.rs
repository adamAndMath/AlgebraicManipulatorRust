use envs::*;
use exp::Exp;
use ty::{ Variance::*, * };
use element::Element;


#[test]
fn struct_empty() {
    let mut exps = vec!();
    let mut tys = vec!();
    {
        let mut env = Envs::new(&mut exps, &mut tys);
        element!(struct Test).define(&mut env).unwrap();

        let e_id = env.exp.get_id("Test").unwrap();
        let ty_id = ttype!(Test).to_id(&env.local()).unwrap();
        let mut type_val = TypeVal::new(vec!());
        type_val.push_atom(e_id);

        assert_eq!(env.exp.get(e_id), Some(&ExpVal::new_empty(ty_id)));
        assert_eq!(env.ty.get_id("Test").and_then(|id|env.ty.get(id)), Some(&type_val))
    }

    assert_eq!(exps.len(), 1);
    assert_eq!(tys.len(), 1);
}

#[test]
fn struct_tuple() {
    let mut exps = vec!();
    let mut tys = vec!();
    {
        let mut env = Envs::new(&mut exps, &mut tys);
        env.ty.add("fn".to_owned(), TypeVal::new(vec!(Contravariant, Covariant)));
        element!(struct A).define(&mut env).unwrap();
        element!(struct B).define(&mut env).unwrap();
        element!(struct Test(A, B)).define(&mut env).unwrap();

        let e_id = env.exp.get_id("Test").unwrap();
        let ty_id = ttype!(fn[(A, B), Test]).to_id(&env.local()).unwrap();
        let mut type_val = TypeVal::new(vec!());
        type_val.push_comp(e_id);

        assert_eq!(env.exp.get(e_id), Some(&ExpVal::new_empty(ty_id)));
        assert_eq!(env.ty.get_id("Test").and_then(|id|env.ty.get(id)), Some(&type_val))
    }

    assert_eq!(exps.len(), 3);
    assert_eq!(tys.len(), 4);
}

#[test]
fn letting() {
    let mut exps = vec!();
    let mut tys = vec!();
    {
        let mut env = Envs::new(&mut exps, &mut tys);
        env.ty.add("fn".to_owned(), TypeVal::new(vec!(Contravariant, Covariant)));
        element!(enum Nat { Zero, Succ(Nat) }).define(&mut env).unwrap();
        element!(let two = Succ(Succ(Zero))).define(&mut env).unwrap();
        element!(let two_marked: Nat = Succ(Succ(Zero))).define(&mut env).unwrap();

        assert_eq!(env.exp.get_id("two").and_then(|id|env.exp.get(id)), env.exp.get_id("two_marked").and_then(|id|env.exp.get(id)));
    }

    assert_eq!(exps.len(), 4);
    assert_eq!(tys.len(), 2);
}
