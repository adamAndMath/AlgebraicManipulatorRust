use predef::*;
use env::LocalID;
use id::renamed::ExpID;
use ast::{ Type, Pattern, Exp, ToID };
use tree::*;

#[test]
fn succ_zero() {
    let mut space = predef_space();

    space.types.add("Nat".to_owned());
    let zero_id = space.exps.add("Zero".to_owned());
    let succ_id = space.exps.add("Succ".to_owned());

    let exp = exp!(Succ(Zero));

    assert_eq!(exp.to_id(&space.local()), Ok(exp_id!(succ_id(zero_id))));
}

#[test]
fn apply() {
    let space = predef_space();
    let e = exp!(forall((x: Bool) -> exists((y: Bool) -> eq(x, y)))).to_id(&space.local()).unwrap();
    let x = LocalID::new(1);
    let y = LocalID::new(0);
    macro_rules! test_apply {
        ($e:ident[$($t:tt)*]{$($r:tt)*} = $($rest:tt)*) =>
            (assert_eq!($e.apply(&tree!([$($t)*]), 0, &|_,_|Ok(exp_id!($($r)*))), exp!($($rest)*).to_id(&space.local()).map_err(Ok), stringify!([$($t)*])));
    }
    test_apply!(e[] {TRUE_ID} = true);
    test_apply!(e[f] {EXISTS_ID} = exists((x: Bool) -> exists((y: Bool) -> eq(x, y))));
    test_apply!(e[t] {TRUE_ID} = forall(true));
    test_apply!(e[0] {TRUE_ID} = forall(true));
    test_apply!(e[0,0] {TRUE_ID} = forall((x: Bool) -> true));
    test_apply!(e[0,0,f] {FORALL_ID} = forall((x: Bool) -> forall((y: Bool) -> eq(x, y))));
    test_apply!(e[0,0,t] {TRUE_ID} = forall((x: Bool) -> exists(true)));
    test_apply!(e[0,0,0] {TRUE_ID} = forall((x: Bool) -> exists(true)));
    test_apply!(e[0,0,0,0] {TRUE_ID} = forall((x: Bool) -> exists((y: Bool) -> true)));
    test_apply!(e[0,0,0,0,t] {(y,x)} = forall((x: Bool) -> exists((y: Bool) -> eq(y, x))));
    test_apply!(e[0,0,0,0,0] {y} = forall((x: Bool) -> exists((y: Bool) -> eq(y, y))));
    test_apply!(e[0,0,0,0,1] {x} = forall((x: Bool) -> exists((y: Bool) -> eq(x, x))));
}
