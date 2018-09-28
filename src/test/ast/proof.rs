use predef::*;
use envs::*;
use ast::*;
use id::renamed::*;
use tree::{ Tree, TreeChar };

#[test]
fn deref() {
    let (mut exps, mut tys, mut truths) = predef();
    let mut env = Envs::new(&mut exps, &mut tys, &mut truths);
    let e_id = exp_id!(TRUE_ID);
    env.truth.add("a".to_owned(), TruthVal::new(e_id.clone()));
    let p = proof!(a());
    let re = p.to_id(&env.local()).unwrap().execute(&env.local(), &MatchEnv::new());
    assert_eq!(re, Ok(e_id));
}

#[test]
fn replace_nothing() {
    let (mut exps, mut tys, mut truths) = predef();
    let mut env = Envs::new(&mut exps, &mut tys, &mut truths);
    alias_predef(&mut env);
    let e = exp!(forall((a: Bool) -> true));
    let e_id = e.to_id(&env.local()).unwrap();
    env.truth.add("b".to_owned(), TruthVal::new(e_id));
    let p = proof!(b(false));
    let re = p.to_id(&env.local()).unwrap().execute(&env.local(), &MatchEnv::new());
    assert_eq!(re, Ok(exp_id!(TRUE_ID)));
}

#[test]
fn replace() {
    let (mut exps, mut tys, mut truths) = predef();
    let mut env = Envs::new(&mut exps, &mut tys, &mut truths);
    alias_predef(&mut env);
    let e = exp!(forall((a: Bool) -> a));
    let e_id = e.to_id(&env.local()).unwrap();
    env.truth.add("b".to_owned(), TruthVal::new(e_id));
    let p = proof!(b(false));
    let re = p.to_id(&env.local()).unwrap().execute(&env.local(), &MatchEnv::new());
    assert_eq!(re, Ok(exp_id!(FALSE_ID)));
}

#[test]
fn unwraping_var() {
    let (mut exps, mut tys, mut truths) = predef();
    let mut env = Envs::new(&mut exps, &mut tys, &mut truths);
    alias_predef(&mut env);
    env.exp.add("x".to_owned(), ExpVal::new(exp_id!(TRUE_ID), type_id!(BOOL_ID), 0));
    let e = exp!(forall((a: Bool) -> x));
    let e_id = e.to_id(&env.local()).unwrap();
    env.truth.add("b".to_owned(), TruthVal::new(e_id));
    let p = proof!(b(false)~wrap(x)[]);
    let re = p.to_id(&env.local()).unwrap().execute(&env.local(), &MatchEnv::new());
    assert_eq!(re, Ok(exp_id!(TRUE_ID)));
}

#[test]
fn unwraping_match() {
    let (mut exps, mut tys, mut truths) = predef();
    let mut env = Envs::new(&mut exps, &mut tys, &mut truths);
    alias_predef(&mut env);
    let e = exp!(match((true, false), {
        (true, true) => true,
        (true, false) => false,
        (false, true) => false,
        (false, false) => false
    }));
    let e_id = e.to_id(&env.local()).unwrap();
    env.truth.add("m".to_owned(), TruthVal::new(e_id));
    let p = proof!(m()~wrap(match((true, false), {
        (true, true) => true,
        (true, false) => false,
        (false, true) => false,
        (false, false) => false
    }))[]);
    let re = p.to_id(&env.local()).unwrap().execute(&env.local(), &MatchEnv::new());
    assert_eq!(re, Ok(exp_id!(FALSE_ID)));
}

#[test]
fn unwraping_lambda_call() {
    let (mut exps, mut tys, mut truths) = predef();
    let mut env = Envs::new(&mut exps, &mut tys, &mut truths);
    alias_predef(&mut env);
    let e = exp!(((a: Bool, b: Bool) -> b)(true, false));
    let e_id = e.to_id(&env.local()).unwrap();
    env.truth.add("m".to_owned(), TruthVal::new(e_id));
    let p = proof!(m()~wrap(((a: Bool, b: Bool) -> b)(true, false))[]);
    let re = p.to_id(&env.local()).unwrap().execute(&env.local(), &MatchEnv::new());
    assert_eq!(re, Ok(exp_id!(FALSE_ID)));
}

#[test]
fn unwraping_function_call() {
    let (mut exps, mut tys, mut truths) = predef();
    let mut env = Envs::new(&mut exps, &mut tys, &mut truths);
    alias_predef(&mut env);
    element!(fn f(a: Bool, b: Bool) = b).define(&mut env).unwrap();
    let e = exp!(f(true, false));
    let e_id = e.to_id(&env.local()).unwrap();
    env.truth.add("m".to_owned(), TruthVal::new(e_id));
    let p = proof!(m()~wrap(f(true, false))[]);
    let re = p.to_id(&env.local()).unwrap().execute(&env.local(), &MatchEnv::new());
    assert_eq!(re, Ok(exp_id!(FALSE_ID)));
}

#[test]
fn match_proof() {
    let (mut exps, mut tys, mut truths) = predef();
    let mut env = Envs::new(&mut exps, &mut tys, &mut truths);
    alias_predef(&mut env);
    let x = env.exp.add("x".to_owned(), ExpVal::new_empty(type_id!(BOOL_ID), 0));
    let e = exp!(match(x, {
        true => true,
        false => false
    }));
    let e_id = e.to_id(&env.local()).unwrap();
    env.truth.add("m".to_owned(), TruthVal::new(e_id));
    let p = proof!(
        match x {
            true => m().match(x)[f]~wrap(match(true, { true => true, false => false }))[]~match(x)[],
            false => m().match(x)[f]~wrap(match(false, { true => true, false => false }))[]~match(x)[]
        }
    );
    let re = p.to_id(&env.local()).unwrap().execute(&env.local(), &MatchEnv::new());
    assert_eq!(re, Ok(exp_id!(x)));
}
