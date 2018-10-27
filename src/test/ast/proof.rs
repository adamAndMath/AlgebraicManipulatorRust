use predef::*;
use envs::*;
use ast::*;
use id::renamed::*;
use tree::Tree;

#[test]
fn deref() {
    let mut space = predef_space();
    let mut env = predef();
    let e_id = exp_id!(TRUE_ID);
    space.truths.add("a".to_owned());
    env.truth.add(TruthVal::new(e_id.clone(), 0));
    let p = proof!(a());
    let re = p.to_id(&space.local()).unwrap().execute(&env.local(), &MatchEnv::new());
    assert_eq!(re, Ok(e_id));
}

#[test]
fn replace_nothing() {
    let mut space = predef_space();
    let mut env = predef();
    let e = exp!(forall((a: Bool) -> true));
    let e_id = e.to_id(&space.local()).unwrap();
    space.truths.add("b".to_owned());
    env.truth.add(TruthVal::new(e_id, 0));
    let p = proof!(b(false));
    let re = p.to_id(&space.local()).unwrap().execute(&env.local(), &MatchEnv::new());
    assert_eq!(re, Ok(exp_id!(TRUE_ID)));
}

#[test]
fn replace() {
    let mut space = predef_space();
    let mut env = predef();
    let e = exp!(forall((a: Bool) -> a));
    let e_id = e.to_id(&space.local()).unwrap();
    space.truths.add("b".to_owned());
    env.truth.add(TruthVal::new(e_id, 0));
    let p = proof!(b(false));
    let re = p.to_id(&space.local()).unwrap().execute(&env.local(), &MatchEnv::new());
    assert_eq!(re, Ok(exp_id!(FALSE_ID)));
}

#[test]
fn unwraping_var() {
    let mut space = predef_space();
    let mut env = predef();
    space.exps.add("x".to_owned());
    env.exp.add(ExpVal::new(exp_id!(TRUE_ID), type_id!(BOOL_ID), 0));
    let e = exp!(forall((a: Bool) -> x));
    let e_id = e.to_id(&space.local()).unwrap();
    space.truths.add("b".to_owned());
    env.truth.add(TruthVal::new(e_id, 0));
    let p = proof!(b(false).def(x)[]);
    let re = p.to_id(&space.local()).unwrap().execute(&env.local(), &MatchEnv::new());
    assert_eq!(re, Ok(exp_id!(TRUE_ID)));
}

#[test]
fn unwraping_match() {
    let mut space = predef_space();
    let mut env = predef();
    let e = exp!({
        (true, true) => true,
        (true, false) => false,
        (false, true) => false,
        (false, false) => false
    }(true, false));
    let e_id = e.to_id(&space.local()).unwrap();
    space.truths.add("m".to_owned());
    env.truth.add(TruthVal::new(e_id, 0));
    let p = proof!(m().def({
        (true, true) => true,
        (true, false) => false,
        (false, true) => false,
        (false, false) => false
    }(true, false))[]);
    let re = p.to_id(&space.local()).unwrap().execute(&env.local(), &MatchEnv::new());
    assert_eq!(re, Ok(exp_id!(FALSE_ID)));
}

#[test]
fn unwraping_lambda_call() {
    let mut space = predef_space();
    let mut env = predef();
    let e = exp!(((a: Bool, b: Bool) -> b)(true, false));
    let e_id = e.to_id(&space.local()).unwrap();
    space.truths.add("m".to_owned());
    env.truth.add(TruthVal::new(e_id, 0));
    let p = proof!(m().def(((a: Bool, b: Bool) -> b)(true, false))[]);
    let re = p.to_id(&space.local()).unwrap().execute(&env.local(), &MatchEnv::new());
    assert_eq!(re, Ok(exp_id!(FALSE_ID)));
}

#[test]
fn unwraping_function_call() {
    let mut space = predef_space();
    let mut env = predef();
    script!{space, env,
        fn f(a: Bool, b: Bool) = b;
    }
    let e = exp!(f(true, false));
    let e_id = e.to_id(&space.local()).unwrap();
    space.truths.add("m".to_owned());
    env.truth.add(TruthVal::new(e_id, 0));
    let p = proof!(m().def(f(true, false))[]);
    let re = p.to_id(&space.local()).unwrap().execute(&env.local(), &MatchEnv::new());
    assert_eq!(re, Ok(exp_id!(FALSE_ID)));
}

#[test]
fn match_proof() {
    let mut space = predef_space();
    let mut env = predef();
    space.exps.add("x".to_owned());
    let x = env.exp.add(ExpVal::new_empty(type_id!(BOOL_ID), 0));
    let e = exp!({
        true => true,
        false => false
    }(x));
    let e_id = e.to_id(&space.local()).unwrap();
    space.truths.add("m".to_owned());
    env.truth.add(TruthVal::new(e_id, 0));
    let p = proof!(
        match x {
            true => m().match(x)[0].def({ true => true, false => false }(true))[]~match(x)[],
            false => m().match(x)[0].def({ true => true, false => false }(false))[]~match(x)[]
        }
    );
    let re = p.to_id(&space.local()).unwrap().execute(&env.local(), &MatchEnv::new());
    assert_eq!(re, Ok(exp_id!(x)));
}

#[test]
fn id_call() {
    let space = predef_space();
    let env = predef();
    let p = proof!(ID[Bool](true));
    let re = p.to_id(&space.local()).unwrap().execute(&env.local(), &MatchEnv::new());
    assert_eq!(re, Ok(exp_id!(EQ_ID[BOOL_ID](TRUE_ID, TRUE_ID))));
}

#[test]
fn double_negate() {
    let mut space = predef_space();
    let mut env = predef();
    script!{space, env,
        fn not {
            true => false,
            false => true
        }

        proof DoubleNegate(b: Bool) {
            match b {
                true => ID[Bool](not(not(true))).def(not(true))[1,0].def(not(false))[1]~match(b)[0,0,0|1],
                false => ID[Bool](not(not(false))).def(not(false))[1,0].def(not(true))[1]~match(b)[0,0,0|1]
            }
        }
    }
    assert_eq!(env.truth[space.get_truth(&path!(DoubleNegate)).unwrap()], TruthVal::new(exp!(forall[Bool]((b: Bool) -> eq[Bool](not(not(b)), b))).to_id(&space.local()).unwrap(), 0));
}