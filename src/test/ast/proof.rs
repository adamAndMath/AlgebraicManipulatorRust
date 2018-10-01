use predef::*;
use envs::*;
use ast::*;
use id::renamed::*;
use tree::Tree;

#[test]
fn deref() {
    let (mut exps, mut tys, mut truths) = predef();
    let mut env = Envs::new(&mut exps, &mut tys, &mut truths);
    let e_id = exp_id!(TRUE_ID);
    env.truth.add("a".to_owned(), TruthVal::new(e_id.clone(), 0));
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
    env.truth.add("b".to_owned(), TruthVal::new(e_id, 0));
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
    env.truth.add("b".to_owned(), TruthVal::new(e_id, 0));
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
    env.truth.add("b".to_owned(), TruthVal::new(e_id, 0));
    let p = proof!(b(false).def(x)[]);
    let re = p.to_id(&env.local()).unwrap().execute(&env.local(), &MatchEnv::new());
    assert_eq!(re, Ok(exp_id!(TRUE_ID)));
}

#[test]
fn unwraping_match() {
    let (mut exps, mut tys, mut truths) = predef();
    let mut env = Envs::new(&mut exps, &mut tys, &mut truths);
    alias_predef(&mut env);
    let e = exp!({
        (true, true) => true,
        (true, false) => false,
        (false, true) => false,
        (false, false) => false
    }(true, false));
    let e_id = e.to_id(&env.local()).unwrap();
    env.truth.add("m".to_owned(), TruthVal::new(e_id, 0));
    let p = proof!(m().def({
        (true, true) => true,
        (true, false) => false,
        (false, true) => false,
        (false, false) => false
    }(true, false))[]);
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
    env.truth.add("m".to_owned(), TruthVal::new(e_id, 0));
    let p = proof!(m().def(((a: Bool, b: Bool) -> b)(true, false))[]);
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
    env.truth.add("m".to_owned(), TruthVal::new(e_id, 0));
    let p = proof!(m().def(f(true, false))[]);
    let re = p.to_id(&env.local()).unwrap().execute(&env.local(), &MatchEnv::new());
    assert_eq!(re, Ok(exp_id!(FALSE_ID)));
}

#[test]
fn match_proof() {
    let (mut exps, mut tys, mut truths) = predef();
    let mut env = Envs::new(&mut exps, &mut tys, &mut truths);
    alias_predef(&mut env);
    let x = env.exp.add("x".to_owned(), ExpVal::new_empty(type_id!(BOOL_ID), 0));
    let e = exp!({
        true => true,
        false => false
    }(x));
    let e_id = e.to_id(&env.local()).unwrap();
    env.truth.add("m".to_owned(), TruthVal::new(e_id, 0));
    let p = proof!(
        match x {
            true => m().match(x)[0].def({ true => true, false => false }(true))[]~match(x)[],
            false => m().match(x)[0].def({ true => true, false => false }(false))[]~match(x)[]
        }
    );
    let re = p.to_id(&env.local()).unwrap().execute(&env.local(), &MatchEnv::new());
    assert_eq!(re, Ok(exp_id!(x)));
}

#[test]
fn id_call() {
    let (mut exps, mut tys, mut truths) = predef();
    let mut env = Envs::new(&mut exps, &mut tys, &mut truths);
    alias_predef(&mut env);
    let p = proof!(ID[Bool](true));
    let re = p.to_id(&env.local()).unwrap().execute(&env.local(), &MatchEnv::new());
    assert_eq!(re, Ok(exp_id!(EQ_ID[BOOL_ID](TRUE_ID, TRUE_ID))));
}

#[test]
fn double_negate() {
    let (mut exps, mut tys, mut truths) = predef();
    let mut env = Envs::new(&mut exps, &mut tys, &mut truths);
    alias_predef(&mut env);
    element!(fn not { true => false, false => true }).define(&mut env).unwrap();
    element!(
        proof DoubleNegate(b: Bool) {
            match b {
                true => ID[Bool](not(not(true))).def(not(true))[1,0].def(not(false))[1]~match(b)[0,0,0|1],
                false => ID[Bool](not(not(false))).def(not(false))[1,0].def(not(true))[1]~match(b)[0,0,0|1]
            }
        }
    ).define(&mut env).unwrap();
    assert_eq!(env.truth.get(env.truth.get_id("DoubleNegate").unwrap()).unwrap(), &TruthVal::new(exp!(forall[Bool]((b: Bool) -> eq[Bool](not(not(b)), b))).to_id(&env.local()).unwrap(), 0));
}