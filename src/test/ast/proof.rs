use parser::Parse;
use predef::*;
use env::{ Path, PushID };
use envs::*;
use ast::*;
use id::renamed::*;

#[test]
fn deref() {
    let mut names = NameData::new();
    let mut space = predef_space(&mut names);
    let mut data = predef_data();
    let mut env = Envs::new(&mut data);
    let e_id = exp_id!(TRUE_ID).push_id(1);
    space.truths.add(&"a");
    env.truth.add(TruthVal::new(e_id.clone(), 0));
    let p = Proof::parse("a()");
    let re = p.to_id(&space).unwrap().execute(&env, &MatchEnv::new());
    assert_eq!(re, Ok(e_id));
}

#[test]
fn replace_nothing() {
    let mut names = NameData::new();
    let mut space = predef_space(&mut names);
    let mut data = predef_data();
    let mut env = Envs::new(&mut data);
    let e = Exp::parse("forall((a: Bool) -> true)");
    let e_id = e.to_id(&space).unwrap().push_id(1);
    space.truths.add(&"b");
    env.truth.add(TruthVal::new(e_id, 0));
    let p = Proof::parse("b(false)");
    let re = p.to_id(&space).unwrap().execute(&env, &MatchEnv::new());
    assert_eq!(re, Ok(exp_id!(TRUE_ID)));
}

#[test]
fn replace() {
    let mut names = NameData::new();
    let mut space = predef_space(&mut names);
    let mut data = predef_data();
    let mut env = Envs::new(&mut data);
    let e = Exp::parse("forall((a: Bool) -> a)");
    let e_id = e.to_id(&space).unwrap().push_id(1);
    space.truths.add(&"b");
    env.truth.add(TruthVal::new(e_id, 0));
    let p = Proof::parse("b(false)");
    let re = p.to_id(&space).unwrap().execute(&env, &MatchEnv::new());
    assert_eq!(re, Ok(exp_id!(FALSE_ID)));
}

#[test]
fn unwraping_var() {
    let mut names = NameData::new();
    let mut space = predef_space(&mut names);
    let mut data = predef_data();
    let mut env = Envs::new(&mut data);
    space.exps.add(&"x");
    env.exp.add(ExpVal::new(exp_id!(TRUE_ID), type_id!(BOOL_ID), 0));
    let e = Exp::parse("forall((a: Bool) -> x)");
    let e_id = e.to_id(&space).unwrap().push_id(1);
    space.truths.add(&"b");
    env.truth.add(TruthVal::new(e_id, 0));
    let p = Proof::parse("b(false).def(x)[]");
    let re = p.to_id(&space).unwrap().execute(&env, &MatchEnv::new());
    assert_eq!(re, Ok(exp_id!(TRUE_ID)));
}

#[test]
fn unwraping_match() {
    let mut names = NameData::new();
    let mut space = predef_space(&mut names);
    let mut data = predef_data();
    let mut env = Envs::new(&mut data);
    let e = Exp::parse("{
        (true, true) => true,
        (true, false) => false,
        (false, true) => false,
        (false, false) => false,
    }(true, false)");
    let e_id = e.to_id(&space).unwrap().push_id(1);
    space.truths.add(&"m");
    env.truth.add(TruthVal::new(e_id, 0));
    let p = Proof::parse("m.def({
        (true, true) => true,
        (true, false) => false,
        (false, true) => false,
        (false, false) => false,
    }(true, false))[]");
    let re = p.to_id(&space).unwrap().execute(&env, &MatchEnv::new());
    assert_eq!(re, Ok(exp_id!(FALSE_ID)));
}

#[test]
fn unwraping_lambda_call() {
    let mut names = NameData::new();
    let mut space = predef_space(&mut names);
    let mut data = predef_data();
    let mut env = Envs::new(&mut data);
    let e = Exp::parse("((a: Bool, b: Bool) -> b)(true, false)");
    let e_id = e.to_id(&space).unwrap().push_id(1);
    space.truths.add(&"m");
    env.truth.add(TruthVal::new(e_id, 0));
    let p = Proof::parse("m.def(((a: Bool, b: Bool) -> b)(true, false))[]");
    let re = p.to_id(&space).unwrap().execute(&env, &MatchEnv::new());
    assert_eq!(re, Ok(exp_id!(FALSE_ID)));
}

#[test]
fn unwraping_function_call() {
    let mut names = NameData::new();
    let mut space = predef_space(&mut names);
    let mut data = predef_data();
    let mut env = Envs::new(&mut data);
    Element::parse("fn f(a: Bool, b: Bool) = b;").to_id_mut(&mut space).unwrap().define(&mut env).unwrap();
    let e = Exp::parse("f(true, false)");
    let e_id = e.to_id(&space).unwrap().push_id(1);
    space.truths.add(&"m");
    env.truth.add(TruthVal::new(e_id, 0));
    let p = Proof::parse("m.def(f(true, false))[]");
    let re = p.to_id(&space).unwrap().execute(&env, &MatchEnv::new());
    assert_eq!(re, Ok(exp_id!(FALSE_ID)));
}

#[test]
fn match_proof() {
    let mut names = NameData::new();
    let mut space = predef_space(&mut names);
    let mut data = predef_data();
    let mut env = Envs::new(&mut data);
    space.exps.add(&"x");
    let x = env.exp.add(ExpVal::new_empty(type_id!(BOOL_ID), 0));
    let e = Exp::parse("{
        true => true,
        false => false
    }(x)");
    let e_id = e.to_id(&space).unwrap().push_id(1);
    space.truths.add(&"m");
    env.truth.add(TruthVal::new(e_id, 0));
    let p = Proof::parse(
        "match x {
            true => m.match(x)[0].def({ true => true, false => false }(true))[]~match(x)[],
            false => m.match(x)[0].def({ true => true, false => false }(false))[]~match(x)[]
        }"
    );
    let re = p.to_id(&space).unwrap().execute(&env, &MatchEnv::new());
    assert_eq!(re, Ok(exp_id!(x)));
}

#[test]
fn id_call() {
    let mut names = NameData::new();
    let space = predef_space(&mut names);
    let mut data = predef_data();
    let env = Envs::new(&mut data);
    let p = Proof::parse("ID<Bool>(true)");
    let re = p.to_id(&space).unwrap().execute(&env, &MatchEnv::new());
    assert_eq!(re, Ok(exp_id!(EQ_ID[BOOL_ID](TRUE_ID, TRUE_ID))));
}

#[test]
fn block() {
    let mut names = NameData::new();
    let space = predef_space(&mut names);
    let mut data = predef_data();
    let env = Envs::new(&mut data);
    let p = Proof::parse("{
        let t = def({ true => false, false => true }(true))
        let f = def({ true => true, false => false }(false))
        f~t[1]
    }");
    let p = p.to_id(&space).unwrap().execute(&env, &MatchEnv::new()).unwrap();
    assert_eq!(p, Exp::parse("eq<Bool>({ true => true, false => false }(false), { true => false, false => true }(true))").to_id(&space).unwrap())
}

#[test]
fn double_negate() {
    let mut names = NameData::new();
    let mut space = predef_space(&mut names);
    let mut data = predef_data();
    let mut env = Envs::new(&mut data);
    Element::parse(
        "fn not {
            true => false,
            false => true
        }"
    ).to_id_mut(&mut space).unwrap().define(&mut env).unwrap();
    Element::parse(
        "proof DoubleNegate(b: Bool) = match b {
            true => ID<Bool>(not(not(true))).def(not(true))[1,0].def(not(false))[1]~match(b)[0,0,0|1],
            false => ID<Bool>(not(not(false))).def(not(false))[1,0].def(not(true))[1]~match(b)[0,0,0|1]
        }"
    ).to_id_mut(&mut space).unwrap().define(&mut env).unwrap();
    assert_eq!(env.truth[space.get_truth(&Path::parse("DoubleNegate")).unwrap()], TruthVal::new(Exp::parse("forall<Bool>((b: Bool) -> eq<Bool>(not(not(b)), b))").to_id(&space).unwrap().push_id(1), 0));
}