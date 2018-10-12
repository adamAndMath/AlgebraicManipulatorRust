use parser::Parse;
use predef::*;
use envs::*;
use env::LocalID;
use id::renamed::ExpID;
use ast::{ Type, Exp, ToID };
use tree::*;

#[test]
fn succ_zero() {
    let (mut exps, mut tys, mut truths) = predef();
    let mut env = Envs::new(&mut exps, &mut tys, &mut truths);
    env.ty.alias("fn".to_owned(), FN_ID.into());

    let nat_id = env.ty.add("Nat".to_owned(), TypeVal::new(vec!()));

    let zero_ty = Type::parse("Nat").to_id(&env.local()).unwrap();
    let zero_id = env.exp.add("Zero".to_owned(), ExpVal::new_empty(zero_ty, 0));
    env.ty.get_mut(nat_id).unwrap().push_atom(zero_id);

    let succ_ty = Type::parse("fn<Nat, Nat>").to_id(&env.local()).unwrap();
    let succ_id = env.exp.add("Succ".to_owned(), ExpVal::new_empty(succ_ty, 0));
    env.ty.get_mut(nat_id).unwrap().push_comp(succ_id);

    let exp = Exp::parse("Succ(Zero)");

    assert_eq!(exp.to_id(&env.local()), Ok(exp_id!(succ_id(zero_id))));
}

#[test]
fn apply() {
    let (mut exps, mut tys, mut truths) = predef();
    let mut env = Envs::new(&mut exps, &mut tys, &mut truths);
    alias_predef(&mut env);
    let e = Exp::parse("forall((x: Bool) -> exists((y: Bool) -> eq(x, y)))").to_id(&mut env.local()).unwrap();
    let x = LocalID::new(1);
    let y = LocalID::new(0);
    macro_rules! test_apply {
        ($env:ident, $e:ident[$($t:tt)*]{$($r:tt)*} = $($rest:tt)*) =>
            (assert_eq!($e.apply(&Tree::parse(stringify!([$($t)*])), 0, &|_,_|Ok(exp_id!($($r)*))), Exp::parse(stringify!($($rest)*)).to_id(&env.local()).map_err(Ok), stringify!([$($t)*])));
    }
    test_apply!(env, e[] {TRUE_ID} = true);
    test_apply!(env, e[f] {EXISTS_ID} = exists((x: Bool) -> exists((y: Bool) -> eq(x, y))));
    test_apply!(env, e[t] {TRUE_ID} = forall(true));
    test_apply!(env, e[0] {TRUE_ID} = forall(true));
    test_apply!(env, e[0,0] {TRUE_ID} = forall((x: Bool) -> true));
    test_apply!(env, e[0,0,f] {FORALL_ID} = forall((x: Bool) -> forall((y: Bool) -> eq(x, y))));
    test_apply!(env, e[0,0,t] {TRUE_ID} = forall((x: Bool) -> exists(true)));
    test_apply!(env, e[0,0,0] {TRUE_ID} = forall((x: Bool) -> exists(true)));
    test_apply!(env, e[0,0,0,0] {TRUE_ID} = forall((x: Bool) -> exists((y: Bool) -> true)));
    test_apply!(env, e[0,0,0,0,t] {(y,x)} = forall((x: Bool) -> exists((y: Bool) -> eq(y, x))));
    test_apply!(env, e[0,0,0,0,0] {y} = forall((x: Bool) -> exists((y: Bool) -> eq(y, y))));
    test_apply!(env, e[0,0,0,0,1] {x} = forall((x: Bool) -> exists((y: Bool) -> eq(x, x))));
}
